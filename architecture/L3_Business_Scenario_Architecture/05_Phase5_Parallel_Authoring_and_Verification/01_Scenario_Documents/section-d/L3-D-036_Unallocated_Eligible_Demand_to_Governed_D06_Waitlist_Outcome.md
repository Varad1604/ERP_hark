# Unallocated Eligible Demand to Governed D06 Waitlist Outcome

**Scenario ID:** L3-D-036  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** C, D  
**Scenario Type:** Simple  
**Priority:** P2  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0757 / CAN-L3-D-037 (final L3-D-036)

---

# 2. Scenario Purpose

Govern approved but unallocated flexible-learning demand through deterministic D06 waitlisting, promotion, expiry or removal.

# 3. Business Outcome

The demand receives an auditable D06-owned waitlist outcome—Queued, Promoted and Allocated, Expired or Cancelled/Removed—without consuming capacity while queued.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An enrollment is Approved, no seat is available, and waitlisting is enabled.

## 4.2 End Boundary

The waitlist entry is promoted and allocated, expires, or is validly removed, with immutable queue history and the applicable completed event.

## 4.3 In Scope

- Create unique deterministic queue position.
- Revalidate eligibility before promotion.
- Promote when a seat is released.
- Expire or remove under policy while preserving history.

## 4.4 Out of Scope

- Guaranteeing future allocation.
- Consuming seat inventory while queued.
- Changing Section C student or core-registration facts.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D06 | Owns Waitlist Entry, ordering, promotion, expiry, removal and queue history. |
| Section C | Provides authoritative student and eligibility references only. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Enrollment Engine | Creates waitlist entry when approved demand cannot be allocated. | Automated creation under policy. |
| Waitlist Promotion Engine | Selects next eligible entry deterministically and triggers allocation. | Automated execution only. |
| Student | May request self-removal where policy permits. | Own-entry removal only. |
| Academic Administrator | May apply authorized extension, removal or manual promotion. | Policy-bound exception authority. |

# 7. Preconditions and Entry Criteria

- Enrollment is Approved and capacity is exhausted.
- Waitlist is enabled and not full.
- Ordering policy is active.

# 8. Trigger

Seat allocation cannot satisfy otherwise eligible approved demand.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate waitlist policy and create unique position. | Enrollment Engine | WaitlistEntryCreated; Added → Queued. |
| 2 | Maintain position under institutional priority and deterministic tie-breakers. | D06 | Queue remains auditable. |
| 3 | When seat is released, select next entry and revalidate eligibility. | Waitlist Promotion Engine | Eligible promotion candidate or skipped invalid entry. |
| 4 | Promote and atomically allocate available seat. | D06 | Promoted → Allocated; WaitlistedStudentPromoted. |
| 5 | If validity expires, mark Expired and prevent promotion. | Waitlist Scheduler | WaitlistEntryExpired. |
| 6 | For withdrawal, cancellation or authorized action, remove entry and preserve historical position. | Student / Academic Administrator / D06 | WaitlistEntryRemoved. |

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
| Waitlist Entry | No entry / approved unallocated enrollment | Queue, promote and allocate, or terminate by expiry/removal. | Added → Queued → Promoted → Allocated → Closed; alternate Expired or Cancelled | D06 | L2-D06 Aggregate 3; W29–W32 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| WaitlistEntryCreated / WaitlistedStudentPromoted / WaitlistEntryExpired / WaitlistEntryRemoved | D06 | Student-facing, seat-allocation, Section C and audit consumers | Records complete governed waitlist outcome. | L2-D06 W29–W32; Event Catalogue |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| Section C | Open Electives & MOOCs | Provides authoritative student and eligibility references only. | Preserve ownership while enabling the scenario. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Creates waitlist entry when approved demand cannot be allocated. | Enrollment Engine | Automated creation under policy. | Open Electives & MOOCs |
| Selects next eligible entry deterministically and triggers allocation. | Waitlist Promotion Engine | Automated execution only. | Open Electives & MOOCs |
| May request self-removal where policy permits. | Student | Own-entry removal only. | Open Electives & MOOCs |
| May apply authorized extension, removal or manual promotion. | Academic Administrator | Policy-bound exception authority. | Open Electives & MOOCs |

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

- The demand receives an auditable D06-owned waitlist outcome—Queued, Promoted and Allocated, Expired or Cancelled/Removed—without consuming capacity while queued.
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
