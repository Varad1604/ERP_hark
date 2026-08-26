# Infrastructure Reservation Change to Controlled Rescheduling

# 1. Scenario Metadata

**Scenario ID:** L3-G-292  
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

Govern an authorized change to an existing Infrastructure Reservation so the revised reservation remains valid under G10 policy and prior reservation history remains traceable.

# 3. Business Outcome

The reservation is updated according to policy while prior reservation history remains traceable.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised reservation requires modification.

## 4.2 End Boundary

The scenario ends when the permitted reservation change is authoritative, prior reservation history is preserved, and `ReservationModified` has been published.

## 4.3 In Scope

- Change to date, time, purpose, duration or participants as permitted by G10.
- Revalidation of availability, capacity, period, conflicts and reservation policy where materially affected.
- Reservation-history preservation.
- Canonical modification event publication.

## 4.4 Out of Scope

- Creation of a new Reservation.
- Reservation cancellation.
- Academic or other consumer-domain rescheduling decisions.
- Any implementation design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G | Owns the Reservation, its modification and preserved history. |
| Platform Foundation / A | Supplies authorization, event governance and audit capabilities without owning the Reservation. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers the Reservation modification under G10 policy. | Performs the change only where authorized. |

G10 permits authorized users to modify reservations but does not define one universal requester role for every reservation source.

# 7. Preconditions and Entry Criteria

- The Reservation exists under G10 ownership.
- The requested modification is one permitted by G10.
- The resource and revised reservation conditions can be revalidated where applicable.
- The modification is authorized.

# 8. Trigger

An authorised reservation requires modification.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the existing Reservation and the requested change. | Infrastructure Administrator / Infrastructure Management | The change is bounded to one canonical Reservation. |
| 2 | Validate authorization and the applicable reservation policy. | Infrastructure Management | The requested modification is permitted to proceed. |
| 3 | Revalidate affected resource availability, capacity, period and conflicts. | Infrastructure Management | The revised Reservation remains valid under G10 rules. |
| 4 | Make the permitted reservation change while preserving prior history. | Infrastructure Management | The frozen business outcome is achieved. |
| 5 | Publish `ReservationModified`. | Infrastructure Management | The completed modification fact is available for authorized consumers and audit evidence. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Non-time modification | Only purpose or participants change. | Apply authorization and policy validation and revalidate only business conditions materially affected by the change. | Reservation is updated without unnecessary change to its period. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Revised period creates a conflict or violates availability/capacity/policy. | The requested modification cannot become authoritative. | Retain the existing Reservation until a later compliant change is authorized. | Infrastructure Management |
| Authorization fails. | Reservation remains unchanged. | Proceed only after a later valid authorization decision. | Infrastructure Management / Platform Foundation |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Authorized users may change reservation date, time, purpose, duration or participants. | Infrastructure Management | L3 limits modification to source-supported categories. | L2-G10 §26 Reservation Modification |
| Every modification remains auditable. | Infrastructure Management | Prior reservation history cannot be silently overwritten. | L2-G10 §26 |
| Reservation validity includes operational status, availability, capacity, period, conflict and policy rules. | Infrastructure Management | Materially affected conditions are revalidated before the change becomes authoritative. | L2-G10 §26 Reservation Validation |
| `ReservationModified` is the canonical reservation-update event. | Infrastructure Management | L3 uses the exact source event name. | L2-G10 Reservation Events |
| Authorization is mandatory. | Platform Foundation — Authorization | Unauthorized modification cannot proceed. | L2-A04 AP-01 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Reservation | Existing Reservation in a business position where modification is permitted | Changes allowed reservation attributes and preserves history | Reservation remains in its source-defined lifecycle position unless another upstream rule changes it | Infrastructure Management | L2-G10 Reservation lifecycle; §26 |

G10 does not define a special `Rescheduled` lifecycle state, so none is introduced.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `ReservationModified` | Infrastructure Management | Authorized reservation consumers | States that the G10 Reservation was successfully updated. | L2-G10 Reservation Events |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No additional cross-domain hand-off is required to complete this frozen G,A scenario. Authorized consumers may subsequently retrieve the authoritative Reservation without acquiring ownership.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Modify Infrastructure Reservation | Infrastructure Administrator | The change must be authorized and satisfy G10 reservation policy. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty or mandatory approval rule is defined for ordinary reservation modification.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve reservation change history | Prior and revised reservation facts remain traceable. | Infrastructure Management |
| Preserve immutable audit evidence | Audit evidence derived from the completed `ReservationModified` fact. | Platform Foundation — Audit |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Reservation reflects the authorized policy-compliant change.
- Prior reservation history remains traceable.
- `ReservationModified` is published by Infrastructure Management.

## 16.2 Postconditions

- The revised Reservation remains a G10-owned fact.
- Authorized consumers may rely on the current reservation details.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

**Source Assessment:** **PASS**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
