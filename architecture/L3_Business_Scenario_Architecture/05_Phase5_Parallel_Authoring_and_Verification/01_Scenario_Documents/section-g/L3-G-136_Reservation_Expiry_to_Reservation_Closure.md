# Reservation Expiry to Reservation Closure

**Scenario ID:** L3-G-136  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, C, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-136 |
| Canonical Title | Reservation Expiry to Reservation Closure |
| Primary Owner | Library Management |
| Participating Domains | G, C, H |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the complete business outcome in which a library reservation reaches its configured expiry condition without producing the required collection outcome, the reservation is closed according to policy, and the resulting reservation status/history is preserved.

The L2 explicitly supports Reservation Expiry as part of waiting-list information and provides Expired and Cancelled as supported reservation statuses. Reservation status values and queue-processing policies remain configurable.

The scenario does not invent an expiry duration or automatic successor-reservation rule.

## 3. Business Outcome

A reservation that has reached its configured expiry condition is no longer actionable as the active reservation for collection, the reservation is closed according to the authoritative reservation lifecycle, and its history remains preserved.

Where another eligible waiting reservation exists, subsequent allocation follows the configured reservation policy as a separate business outcome.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An existing reservation reaches its configured expiry condition without completing the required collection outcome.

### 4.2 End Boundary

The expired reservation has been processed as no longer active and reaches the authoritative closed outcome.

### 4.3 In Scope

- Identify reservation.
- Confirm reservation status.
- Determine configured expiry condition.
- Confirm that collection did not occur within the applicable period.
- Mark/record expiry.
- Establish reservation closure.
- Preserve reservation history.
- Provide relevant business outcome to H.
- Enable subsequent reservation allocation where policy requires.

### 4.4 Out of Scope

- Creating a new reservation.
- Reordering the waiting list through invented rules.
- Resource circulation issue/return.
- Membership changes.
- Fine calculation/payment unless a separately governed rule applies.
- Notification delivery implementation.
- Technical scheduling/queue mechanics.

## 5. Ownership and Participating Domains

### Library Management — G

Owns Reservation Management, reservation status and closure.

### Student Lifecycle — C

Provides authoritative student context where applicable; C does not own reservation closure.

### Communication — H

Consumes the reservation-expiry/closure business outcome for governed communication.

No ownership transfers.

## 6. Business Actors, Roles and Responsibilities

### Librarian / Circulation Desk Operator

Reviews or manages reservation closure under configured authority.

### Library Member

Is the subject of the expired reservation.

### C Authority

Provides student context where applicable.

### H

Delivers any configured communication associated with reservation expiry/closure.

## 7. Preconditions and Entry Criteria

1. A valid reservation exists.
2. The reservation has not been collected or otherwise closed.
3. A configured reservation expiry condition exists.
4. The expiry condition has been reached.
5. The reservation can be identified authoritatively.

## 8. Trigger

The reservation reaches its configured expiry condition without collection.

## 9. Main Business Flow

### Phase A — Establish Expiry

1. Identify the reservation.
2. Confirm its current reservation condition.
3. Establish the applicable Reservation Expiry information.
4. Confirm that the required collection outcome did not occur within the applicable period.
5. Determine that the reservation has therefore expired.

### Phase B — Close Reservation

6. Record the reservation as Expired according to the configured status model.
7. Establish the reservation's closure outcome under the canonical reservation lifecycle.
8. Preserve queue position, reservation date, priority, notification status, expiry information and relevant history.
9. Make the reservation no longer eligible for ordinary collection under its expired outcome.

### Phase C — Downstream Consequence

10. Provide the expiry/closure business outcome to H where communication is configured.
11. Where other waiting reservations exist, make the resource available for subsequent allocation under the configured queue-processing policy.
12. End the scenario.

The scenario does not create or select the next reservation itself beyond making the closed outcome available to the next governed reservation process.

## 10. Alternate Business Flows

### 10.1 Member Collects Before Expiry Is Effective

If collection occurs before the expiry condition becomes effective, the reservation follows the collection path and this scenario does not close it as expired.

### 10.2 Reservation Already Cancelled

A cancelled reservation does not undergo expiry closure.

### 10.3 Reservation Already Collected

A collected reservation does not undergo expiry closure.

### 10.4 Resource Remains Available

If the resource remains available after expiry, subsequent reservation allocation follows configured waiting-list policy. This scenario does not invent automatic reassignment logic.

### 10.5 Member Remains Eligible for a Future Reservation

The member may submit a new reservation according to configured reservation policy. This scenario does not automatically create a new reservation.

## 11. Failure, Interruption and Business Recovery

### 11.1 Expiry Cannot Be Established

The reservation remains under its current authoritative condition until the business expiry condition can be established.

### 11.2 Collection Status Cannot Be Established

The reservation is not closed as expired until the authoritative collection condition is known.

### 11.3 Closure Authority Is Not Established

The reservation remains governed by the current configured status/expiry policy until the appropriate business authority is available.

### 11.4 Historical Information Is Required

Reservation history is preserved; closure does not erase the reservation's prior waiting/availability information.

No technical timer, scheduler, queue, retry or rollback mechanism is specified.

## 12. Business Rules and Decision Constraints

Waiting-list information includes:

- Queue Position.
- Reservation Date.
- Priority.
- Notification Status.
- Reservation Expiry.

Queue processing policies remain configurable.

Reservation statuses include:

- Requested.
- Waiting.
- Ready for Collection.
- Collected.
- Expired.
- Cancelled.

Status values remain configurable.

The canonical Reservation lifecycle is:

`Requested → Waiting → Available → Collected → Closed`.

This scenario uses the supported Expired operational status while preserving the canonical closure outcome.

No fixed expiry duration, automatic cancellation rule, or automatic reassignment priority is invented.

## 13. State and Lifecycle Impact

The business outcome is:

`Waiting/Available → Expired → Closed`

where **Expired** is an explicitly supported reservation status and **Closed** is the terminal lifecycle condition in the aggregate lifecycle.

The exact preceding lifecycle condition depends on where the reservation was when the expiry condition was reached.

No `Pending Expiry`, `Closure Pending`, or `Expired Pending Notification` state is invented.

## 14. Business Events and Cross-Domain Collaboration

The retrieved L2 service model provides Reservation Services for Reserve Resource, Cancel Reservation, Allocate Reserved Resource and Query Reservation.

The available event material does not expose a dedicated reservation-expiry or reservation-closure event name. Therefore no `ReservationExpired` or `ReservationClosed` event is invented.

Business collaboration:

- C provides student context where applicable.
- H consumes the configured business outcome for communication.
- G remains owner of reservation status and closure.
- Subsequent allocation to another waiting member follows the reservation workflow.

## 15. Business Authorization, Approval and Audit Obligations

Reservation closure follows the configured library authorization model.

The G05 authorization model identifies Librarian and Circulation Desk among the typical operational roles and supports delegated responsibility.

The reservation expiry condition, status change, closure outcome and relevant history must be auditable.

No technical access control/IAM or scheduling implementation is described.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Reservation identified.
- Expiry condition established.
- Collection absence confirmed.
- Reservation marked Expired under supported status semantics.
- Closure outcome established.
- Reservation history preserved.
- Communication outcome made available where configured.
- Resource made available to subsequent reservation processing where applicable.

### Postconditions

- The expired reservation is closed and is no longer an active collection entitlement.
- Historical reservation information remains preserved.
- The member may make a new reservation only through the normal reservation workflow.
- Any subsequent allocation follows configured queue/priority policy.
- H may communicate the expiry/closure outcome.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

The retrieved L2 does not expose a dedicated reservation-expiry or reservation-closure event name. No event is invented.

### Assumptions

- Reservation expiry conditions are configurable.
- Expired is a supported reservation status.
- Closure remains the canonical terminal lifecycle outcome.

### Deferred Questions

Confirm the canonical reservation event vocabulary upstream if dedicated expiry/closure events are required before Architecture Approval/Lock.
