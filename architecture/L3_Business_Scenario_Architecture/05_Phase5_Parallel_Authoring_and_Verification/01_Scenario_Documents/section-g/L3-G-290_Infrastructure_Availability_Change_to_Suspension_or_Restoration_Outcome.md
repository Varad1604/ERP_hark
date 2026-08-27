# Infrastructure Availability Change to Suspension or Restoration Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-G-290  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** D, G  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-D03 — Timetable Workflows; `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and Architecture Approval review

# 2. Scenario Purpose

Govern a change in physical-resource availability so Infrastructure Management remains authoritative for current usability while Academic Delivery consumes that fact for its own scheduling decisions.

# 3. Business Outcome

Availability is suspended or restored under policy and consuming domains receive the authoritative current usability state.

# 4. Scenario Boundary and Scope

The scenario begins when a physical resource becomes unavailable, restricted, or eligible to return to service and ends when G10 has established the current authoritative availability condition, published the canonical availability-change fact, and made that fact available to Academic Delivery without transferring ownership.

In scope: restriction/suspension/restoration, effective scope and period, policy and lifecycle validation, consumer hand-off, event publication and evidence. Out of scope: timetable changes, maintenance execution, invented availability lifecycle states and implementation mechanisms.

# 5. Ownership and Participating Domains

- **G10:** owns the current infrastructure availability fact and governing rules.
- **D03:** consumes authoritative availability for its own scheduling decisions and retains timetable ownership.

# 6. Business Actors, Roles and Responsibilities

**Infrastructure Administrator** performs the governed G10 availability change where authorized. Academic Delivery participants may consume the resulting fact but cannot change G10-owned availability.

# 7. Preconditions and Entry Criteria

- Affected physical resource exists.
- Current lifecycle/availability is known.
- The business condition for restriction, suspension or restoration is established.
- The requested change is permitted by policy and authorization.

# 8. Trigger

A physical resource becomes unavailable, restricted, or eligible to return to service.

# 9. Main Business Flow

1. Identify the resource and the business condition changing its usability.
2. Validate authorization, lifecycle compatibility and applicable availability policy.
3. Apply the authoritative availability change to the correct hierarchy scope and effective period.
4. Make the resulting current usability fact available to Academic Delivery.
5. Publish `InfrastructureAvailabilityChanged`.
6. Academic Delivery evaluates any scheduling consequence within D03; this scenario does not mandate a D03 lifecycle transition.

# 10. Alternate Business Flows

- Partial restriction: apply only to the governed affected scope.
- Scheduled restriction/restoration: govern the applicable future effective period.
- Restoration: restore the source-defined usable availability condition only when the governing business restriction no longer applies.

# 11. Failure, Interruption and Business Recovery

If policy, lifecycle or authorization validation fails, the prior authoritative availability fact remains current. A later change may proceed only after the governing condition is corrected.

# 12. Business Rules and Decision Constraints

- Availability remains independent from physical lifecycle.
- `InfrastructureAvailabilityChanged` is the canonical event for any change to an existing authoritative availability condition, including restriction, business suspension, restoration, scheduled closure, partial closure and other source-defined availability change.
- Lifecycle events do not substitute for availability events.
- D03 may consume the G10 availability fact but cannot mutate it.

# 13. State and Lifecycle Impact

The frozen words “suspended” and “restored” describe business outcomes. The resulting position is the source-defined G10 availability condition appropriate to the business facts; no new `Suspended` or `Restored` lifecycle state is invented. Physical lifecycle remains unchanged.

# 14. Business Events and Cross-Domain Collaboration

- `InfrastructureAvailabilityChanged` — published by G10 for the resulting availability fact.

G10 provides the current authoritative usability fact to D03. D03 retains ownership of timetable/scheduling responses.

# 15. Business Authorization, Approval and Audit Obligations

Infrastructure Administrator must satisfy G10 authorization and availability rules. Evidence preserves prior/current availability, scope/effective period, reason, actor/authority and `InfrastructureAvailabilityChanged` publication.

# 16. Completion Criteria and Postconditions

Completion requires the authoritative G10 usability fact to reflect the governed suspension/restriction or restoration outcome, be available to D03 without ownership transfer, and be represented by `InfrastructureAvailabilityChanged`.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing exact suspension/restoration event was resolved by locked `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md` §5.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
