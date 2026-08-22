# Infrastructure Reservation Cancellation to Resource Availability Restoration

# 1. Scenario Metadata

**Scenario ID:** L3-G-293  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Govern cancellation of a G10 Infrastructure Reservation so its physical resource is released according to availability policy while reservation history remains preserved.

# 3. Business Outcome

The reservation is closed and the applicable resource becomes available according to its availability policy.

The frozen outcome requires the Reservation to be “closed,” but locked G10 defines no `Closed` or `Cancelled` Reservation lifecycle state. The Draft is therefore **BLOCKED** on lifecycle resolution.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised reservation is cancelled.

## 4.2 End Boundary

The scenario ends when the Reservation has reached the source-authorized cancellation/closure lifecycle position, the resource has been released and its availability determined under policy, reservation history is preserved, and `ReservationCancelled` has been published.

## 4.3 In Scope

- Reservation cancellation under G10 governance.
- Release of the reserved physical resource.
- Determination of current availability according to the authoritative availability policy.
- Preservation of reservation history.
- `ReservationCancelled` publication and auditability.

## 4.4 Out of Scope

- Reservation creation or modification.
- Redefinition of availability policy.
- Treating cancellation as deletion.
- Downstream consumer decisions after resource release.
- Any implementation design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G | Owns the Reservation, its cancellation, resource release and resulting availability fact. |
| Platform Foundation / A | Supplies authorization, event governance and audit capabilities without owning the G10 outcome. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers the Reservation cancellation under G10 governance. | Performs cancellation only where authorized. |

No universal named requester or approver is defined for all reservation cancellations.

# 7. Preconditions and Entry Criteria

- The Reservation exists under G10 ownership.
- The referenced physical resource exists.
- Cancellation is authorized.
- An authoritative availability policy can determine the resource's usability after release.

# 8. Trigger

An authorised reservation is cancelled.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the Reservation and its reserved physical resource. | Infrastructure Administrator / Infrastructure Management | The exact reservation commitment to cancel is known. |
| 2 | Validate authorization and applicable reservation-cancellation policy. | Infrastructure Management | The cancellation is eligible to proceed. |
| 3 | Cancel the reservation commitment without deleting its history. | Infrastructure Management | The physical resource is released from that Reservation. |
| 4 | Determine the resource's current availability under its authoritative availability policy. | Infrastructure Management | The resource becomes `Available` only where no other source-defined restriction applies. |
| 5 | Move the Reservation to the lifecycle position required by the frozen “closed” outcome. | Infrastructure Management | **Blocked:** locked G10 does not define that cancellation/closure lifecycle position. |
| 6 | Publish `ReservationCancelled`. | Infrastructure Management | The completed cancellation fact is canonical once the lifecycle conflict is resolved. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Other availability restriction remains | Cancellation removes the reservation constraint but another source-defined condition still restricts use. | Release the Reservation and retain the applicable non-available G10 condition. | Cancellation does not falsely declare the resource usable. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Cancellation is not authorized. | Reservation remains unchanged. | Proceed only after a later valid authorization decision. | Infrastructure Management / Platform Foundation |
| Reservation/resource reference is invalid. | No authoritative cancellation can be applied. | Resolve the G10 business reference before a later attempt. | Infrastructure Management |
| Cancellation lifecycle position remains undefined. | L3 cannot prove the frozen “reservation is closed” outcome using canonical states. | Resolve the locked G10 Reservation lifecycle through architecture change control. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Cancellation releases Infrastructure, restores availability, publishes an event and preserves audit history; Reservation deletion never occurs. | Infrastructure Management | Cancellation is non-destructive and preserves history. | L2-G10 §26 Reservation Cancellation |
| Availability remains independent from lifecycle and follows its own policy. | Infrastructure Management | Resource release does not bypass other availability restrictions. | L2-G10 §25 |
| `ReservationCancelled` is the canonical cancellation event. | Infrastructure Management | L3 preserves the exact source event name. | L2-G10 Reservation Events |
| Authorization is mandatory. | Platform Foundation — Authorization | Unauthorized cancellation cannot proceed. | L2-A04 AP-01 |

# 13. State and Lifecycle Impact

Locked G10 defines the Reservation lifecycle as `Requested → Confirmed → Active → Completed → Archived`. It defines no `Closed` or `Cancelled` state, yet the frozen outcome says the Reservation is closed.

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Reservation | Existing cancellable Reservation position is not further specified | Cancellation/closure is required | **Unresolved:** no canonical cancellation/closure state is defined | Infrastructure Management | L2-G10 Reservation lifecycle; §26 |
| Infrastructure Availability | Existing source-defined condition | Reservation constraint is released | `Available` where policy permits, otherwise another source-defined availability state | Infrastructure Management | L2-G10 §§25–26 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `ReservationCancelled` | Infrastructure Management | Authorized reservation consumers | States that the G10 Reservation was cancelled. | L2-G10 Reservation Events |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No ownership-changing cross-domain hand-off is required. Authorized consumers may subsequently obtain the current G10 reservation/availability facts.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Cancel Infrastructure Reservation | Infrastructure Administrator | Must satisfy authorization and G10 cancellation rules. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty or mandatory approval rule is defined for ordinary reservation cancellation.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve cancellation history | Reservation cancellation and resource release remain historically traceable; the Reservation is not deleted. | Infrastructure Management |
| Preserve immutable audit evidence | Audit evidence derived from `ReservationCancelled`. | Platform Foundation — Audit |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Reservation is in the canonical lifecycle position corresponding to the frozen “closed” outcome.
- The physical resource is released and current availability is determined under policy.
- Reservation history is preserved.
- `ReservationCancelled` is published by Infrastructure Management.

## 16.2 Postconditions

- The resource may be consumed only if its current G10 availability permits it.
- The cancelled Reservation remains historically traceable.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The frozen outcome requires the Reservation to be “closed,” but G10's canonical Reservation lifecycle contains only `Requested`, `Confirmed`, `Active`, `Completed`, and `Archived`. | L3 cannot state the required resulting lifecycle position without inventing or renaming a state. | Upstream G10 must define how cancellation maps to the canonical Reservation lifecycle. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
