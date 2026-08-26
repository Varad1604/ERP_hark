# Unallocated Eligible Demand to Governed D06 Waitlist Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-D-036  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** C, D  
**Scenario Type:** Simple  
**Priority:** P2  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Frozen Stage 3 Source:** P2W-D-0757 / CAN-L3-D-037 (final L3-D-036)

---

# 2. Scenario Purpose

Govern Approved but unallocated flexible-learning demand through D06 waitlisting, promotion, expiry or removal under institutional priority policy.

# 3. Business Outcome

The demand receives an auditable D06-owned waitlist outcome—Queued, Promoted and Allocated, Expired or Cancelled/Removed—without consuming capacity while queued.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An enrollment is Approved, no seat is available and waitlisting is enabled.

## 4.2 End Boundary

The waitlist entry is promoted and allocated, expires, or is validly removed; queue history and applicable D06 business facts remain traceable.

## 4.3 In Scope

- Establish a unique governed queue position.
- Maintain priority/order under D06 policy.
- Revalidate eligibility before promotion.
- Promote when a seat becomes available.
- Expire or remove under policy while preserving history.

## 4.4 Out of Scope

- Guaranteeing future allocation.
- Consuming seat capacity while queued.
- Technical schedulers, promotion engines, atomic allocation or queue implementation.

## 4.5 Related Scenarios — Optional

L3-D-035 produces the unallocated demand; L3-D-048 governs allocation conflict if one is detected.

# 5. Ownership and Participating Domains

D06 owns Waitlist Entry, ordering, promotion, expiry, removal and queue history. Section C supplies authoritative eligibility/student references only.

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Student | May request own waitlist removal where policy permits. | Own-entry action only. |
| Academic Administrator / Academic Affairs, where established by D06 policy | May decide a permitted exceptional extension/removal/manual promotion. | Policy-bound exception authority. |

Normal queue maintenance/promotion/expiry are D06 business responsibilities, not engine/scheduler actors.

# 7. Preconditions and Entry Criteria

Enrollment is Approved, capacity is exhausted, waitlist is enabled and applicable ordering policy is active.

# 8. Trigger

Seat allocation cannot satisfy otherwise eligible Approved demand.

# 9. Main Business Flow

1. D06 establishes a governed Waitlist Entry and queue position — `WaitlistEntryCreated`.
2. D06 maintains position under institutional priority/order policy.
3. When capacity becomes available, D06 revalidates eligibility and determines the next eligible demand.
4. Eligible demand is promoted and allocated — `WaitlistedStudentPromoted`.
5. Where validity expires before promotion, D06 records `WaitlistEntryExpired`.
6. Where the student withdraws/cancels or a valid authorized removal applies, D06 records `WaitlistEntryRemoved`.

# 10. Alternate Business Flows

## 10.1 Expiry

Where waitlist validity expires before promotion, the entry reaches Expired and no allocation occurs.

## 10.2 Valid Removal

Where the student withdraws or authorized policy removes the entry, it reaches Removed/Cancelled with historical queue position preserved.

# 11. Failure, Interruption and Business Recovery

Invalid eligibility or queue-policy references block promotion until resolved. Missing exceptional authority blocks the requested exception. Interrupted waitlist progression preserves the last authoritative D06 queue position/history and continues when business conditions are resolved.

# 12. Business Rules and Decision Constraints

D06 owns waitlist lifecycle; queued demand does not consume capacity; ordering/promotion follows authoritative policy; Section C ownership is unchanged; technical queue/scheduler/atomicity mechanisms are deferred.

# 13. State and Lifecycle Impact

Waitlist Entry: Added → Queued → Promoted → Allocated → Closed; governed alternatives Expired or Cancelled/Removed. Owner D06. Source: L2-D06 Aggregate 3; W29–W32.

# 14. Business Events and Cross-Domain Collaboration

D06 publishes `WaitlistEntryCreated`, `WaitlistedStudentPromoted`, `WaitlistEntryExpired`, and `WaitlistEntryRemoved` as defined in L2-D06 W29–W32/Event Catalogue. Section C supplies authoritative references without ownership transfer.

# 15. Business Authorization, Approval and Audit Obligations

Student may act only on own entry where policy permits. Any exceptional extension/removal/manual promotion requires explicit D06-policy business authority. Audit retains queue position history, eligibility references, policy, decision authority and terminal waitlist outcome.

# 16. Completion Criteria and Postconditions

Waitlist entry has a governed Queued/Promoted+Allocated/Expired/Removed outcome; capacity remains consistent; D06 ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

No new open issue or assumption is introduced. Queue implementation, scheduler design, atomic allocation, retries and persistence are deferred to later levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** L2-D06; Template v2.0; Architecture Level Validation Standards.
