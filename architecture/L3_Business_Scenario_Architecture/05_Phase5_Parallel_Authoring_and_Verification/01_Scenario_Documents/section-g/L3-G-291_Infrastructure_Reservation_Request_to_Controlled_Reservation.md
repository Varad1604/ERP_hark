# Infrastructure Reservation Request to Controlled Reservation

# 1. Scenario Metadata

**Scenario ID:** L3-G-291  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A, D  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-D03 — Timetable Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Establish a governed G10 Infrastructure Reservation for an authorized operational requirement while preserving Academic Delivery ownership of its scheduling process and Platform Foundation ownership of authorization, event governance and audit evidence.

# 3. Business Outcome

A valid Reservation is established for the applicable resource and period.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised consumer requests reservation of available infrastructure.

## 4.2 End Boundary

The scenario ends when G10 has established a valid Reservation for the applicable resource and period and published `InfrastructureReservationCreated`.

## 4.3 In Scope

- Authorized reservation request.
- Resource availability, capacity, period, conflict and reservation-policy validation.
- Creation of the canonical G10 Reservation.
- Recurring-reservation validation where applicable.
- Canonical reservation event publication and auditability.

## 4.4 Out of Scope

- Timetable creation, publication or revision.
- D03's timetable-root `Resource Reservation` record as a substitute for the G10 Reservation.
- Reservation modification or cancellation.
- Transfer of physical-infrastructure ownership.
- Any implementation design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G | Owns the canonical temporary Infrastructure Reservation and its reservation rules. |
| Platform Foundation / A | Supplies authorization, event governance and audit capabilities without owning the Reservation. |
| Academic Delivery / D | May originate/consume an academic infrastructure requirement and retains ownership of timetable decisions. |

D03's root-owned `Resource Reservation` record belongs to the Timetable aggregate; it does not replace or re-own the G10 `Reservation` aggregate governing temporary use of physical infrastructure.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers G10 reservation governance. | Establishes the Reservation only after source-defined validations and authorization. |
| Infrastructure Coordinator | Represents D03's infrastructure scheduling concern. | Verifies room availability and validates infrastructure allocation within D03; does not own the G10 Reservation. |

L2-G10 does not define one universal named requester role for every reservation source.

# 7. Preconditions and Entry Criteria

- The physical resource is operational and available for the requested period.
- Capacity is sufficient.
- The requested period and reservation purpose are valid.
- No conflicting reservation exists unless a source-supported authorized override applies.
- The reservation action is authorized.

# 8. Trigger

An authorised consumer requests reservation of available infrastructure.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Provide the authorized operational requirement for a physical resource and period. | Authorized consumer; Academic Delivery where applicable | A reservation request enters G10 without transferring the consumer's process ownership. |
| 2 | Validate resource availability, capacity, period, conflicts and reservation policy. | Infrastructure Management | The requested reservation is eligible or rejected under G10 rules. |
| 3 | Apply any source-supported recurring pattern or authorized conflict override. | Infrastructure Management | The governed reservation conditions are complete. |
| 4 | Establish the G10 Reservation for the applicable resource and period. | Infrastructure Management | The frozen business outcome is achieved. |
| 5 | Publish `InfrastructureReservationCreated`. | Infrastructure Management | The completed reservation fact is available for authorized consumers and audit evidence. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Recurring reservation | A recurring pattern is requested. | Validate frequency, effective dates and exception dates under reservation policy. | A governed recurring Reservation may be established. | Infrastructure Management |
| Authorized conflict override | A conflict exists and G10 policy permits an authorized override. | Apply only the source-supported override path. | Reservation may proceed without transferring ownership. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Space is unavailable, not operational, insufficient in capacity, or conflicts with another reservation. | No valid Reservation is established. | Correct the business condition or choose another permitted resource/period before a later request. | Infrastructure Management |
| Reservation policy or authorization fails. | The request is rejected. | A later request may proceed only when the governing condition is satisfied. | Infrastructure Management / Platform Foundation |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure Management owns temporary reservation governance. | Infrastructure Management | Consumer domains request/consume reservations but do not own the G10 Reservation. | L2-G10 §26 |
| A reservation requires operational and available space, sufficient capacity, valid period, no conflict and satisfied policy. | Infrastructure Management | Failure of any required validation prevents establishment. | L2-G10 §26 Reservation Validation |
| Conflicting reservations are rejected unless an authorized administrator override is permitted. | Infrastructure Management | Conflict cannot be silently ignored. | L2-G10 §26 |
| D03 owns timetable scheduling and its root-owned Resource Reservation record while consuming G10 physical resources. | Academic Delivery | D03's internal scheduling record does not transfer G10 Reservation ownership. | L2-D03 §§2.7, 17; G10 §26 |
| `InfrastructureReservationCreated` is the canonical successful-reservation event. | Infrastructure Management | L3 uses the exact source event name. | L2-G10 Reservation Events |
| Authorization is mandatory before the business operation proceeds. | Platform Foundation — Authorization | Unauthorized requests cannot establish a Reservation. | L2-A04 AP-01 |

# 13. State and Lifecycle Impact

The G10 Reservation lifecycle is `Requested → Confirmed → Active → Completed → Archived`. The frozen outcome requires establishment of a valid Reservation but does not require L3 to invent a transition beyond the source-supported establishment semantics.

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Reservation | Reservation request | Establishes a valid G10 Reservation | Source-defined confirmed/established reservation position; later lifecycle remains outside this scenario | Infrastructure Management | L2-G10 Reservation lifecycle; §26 |
| Timetable | Existing or absent D03 scheduling context | Supplies/consumes infrastructure requirement only | No D03 transition is caused by G10 establishment alone | Academic Delivery | L2-D03 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `InfrastructureReservationCreated` | Infrastructure Management | Authorized consumers, including Academic Delivery where relevant | States that the canonical Infrastructure Reservation was successfully established. | L2-G10 Reservation Events |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Academic Delivery / authorized consumer | Infrastructure Management | Authorized operational resource-and-period requirement | Requests temporary use without transferring process ownership. | Frozen semantics; L2-D03 |
| Infrastructure Management | Academic Delivery / authorized consumer | Confirmed Infrastructure Reservation fact | Provides the authoritative temporary reservation result. | L2-G10 §26 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Establish Infrastructure Reservation | Infrastructure Administrator | G10 reservation rules and the authorization decision must permit establishment. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No universal separation-of-duty rule is defined for ordinary reservation establishment. A conflict override is permitted only where G10 explicitly allows an authorized administrative override.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve reservation establishment | The canonical Reservation, resource, period and source-supported reservation conditions remain traceable. | Infrastructure Management |
| Preserve immutable audit evidence | Audit evidence derived from `InfrastructureReservationCreated`. | Platform Foundation — Audit |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A valid G10 Reservation exists for the applicable resource and period.
- Required availability, capacity, conflict and policy validations were satisfied.
- `InfrastructureReservationCreated` is published by Infrastructure Management.

## 16.2 Postconditions

- Authorized consumers may rely on the Reservation under G10 ownership.
- D03 retains ownership of its own timetable and scheduling records.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

**Source Assessment:** **PASS**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
