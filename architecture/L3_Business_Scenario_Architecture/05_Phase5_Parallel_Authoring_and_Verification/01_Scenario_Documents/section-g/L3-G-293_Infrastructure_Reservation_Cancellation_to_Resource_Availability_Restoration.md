# Infrastructure Reservation Cancellation to Resource Availability Restoration

# 1. Scenario Metadata

**Scenario ID:** L3-G-293  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A04; L2-A06; L2-A07; `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and Architecture Approval review

# 2. Scenario Purpose

Govern cancellation of a G10 Infrastructure Reservation so its physical resource is released according to availability policy while reservation history remains preserved.

# 3. Business Outcome

The reservation is closed and the applicable resource becomes available according to its availability policy.

# 4. Scenario Boundary and Scope

The scenario begins when an authorized Reservation is cancelled and ends when the Reservation has reached the canonical cancellation outcome, no longer reserves the resource, is historically preserved, the resource availability has been recalculated under G10 policy, and `ReservationCancelled` has been published.

In scope: cancellation, resource release, availability determination, Reservation lifecycle position, history, event and audit evidence. Out of scope: Reservation creation/modification, availability-policy redesign, downstream consumer actions and implementation mechanics.

# 5. Ownership and Participating Domains

- **G10:** owns Reservation cancellation, resource release and resulting availability fact.
- **A:** supplies authorization, event-governance and audit capabilities without owning the business outcome.

# 6. Business Actors, Roles and Responsibilities

**Infrastructure Administrator** performs the governed cancellation where authorized.

# 7. Preconditions and Entry Criteria

- Reservation exists under G10 ownership.
- Referenced physical resource exists.
- Reservation is in `Requested`, `Confirmed` or `Active` where cancellation is permitted.
- Cancellation is authorized.
- Authoritative availability policy is available.

# 8. Trigger

An authorised reservation is cancelled.

# 9. Main Business Flow

1. Resolve the Reservation and physical resource.
2. Validate authorization and cancellation rules.
3. Transition the Reservation from `Requested`, `Confirmed` or `Active` to `Cancelled`.
4. Release the reservation constraint without deleting history.
5. Determine the resource's current Availability Profile; it becomes `Available` only if no other authoritative restriction applies.
6. Publish `ReservationCancelled`.
7. Move the cancelled Reservation to `Archived` when historical preservation processing applies.

# 10. Alternate Business Flows

If another authoritative availability restriction remains, cancellation releases only the reservation constraint; the resource does not become falsely available.

# 11. Failure, Interruption and Business Recovery

Unauthorized cancellation or invalid Reservation/resource reference prevents the transition. Existing history and current authoritative conditions remain unchanged.

# 12. Business Rules and Decision Constraints

- Canonical Reservation lifecycle: `Requested → Confirmed → Active → Completed → Archived`.
- Governed cancellation branch: `Requested / Confirmed / Active → Cancelled → Archived`.
- `Cancelled` is the canonical terminal business outcome for cancellation before normal completion.
- For scenario semantics, a Reservation is **closed** once it reaches `Completed` or `Cancelled` and no longer reserves the resource; `Closed` is not an additional lifecycle state.
- `ReservationCancelled` is the sole canonical cancellation event.
- Cancellation releases the reservation constraint; Availability becomes `Available` only where no other authoritative restriction applies.

# 13. State and Lifecycle Impact

The Reservation enters `Cancelled` and may then be historically preserved at `Archived`. The frozen word “closed” describes the business completion meaning and is not a separate state.

# 14. Business Events and Cross-Domain Collaboration

- `ReservationCancelled` — published by Infrastructure Management.

Authorized consumers may subsequently use the current G10 Reservation/Availability facts without ownership transfer.

# 15. Business Authorization, Approval and Audit Obligations

Infrastructure Administrator must satisfy authorization and G10 cancellation rules. Evidence preserves Reservation/resource identity, prior state, cancellation authority/reason, `Cancelled` result, resource-release/availability result and `ReservationCancelled` publication.

# 16. Completion Criteria and Postconditions

Completion requires the Reservation to be `Cancelled`, no longer reserve the resource, preserve history, publish `ReservationCancelled`, and leave the resource at the availability condition dictated by G10 policy.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing cancellation/closure lifecycle mapping was resolved by locked `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md` §6.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
