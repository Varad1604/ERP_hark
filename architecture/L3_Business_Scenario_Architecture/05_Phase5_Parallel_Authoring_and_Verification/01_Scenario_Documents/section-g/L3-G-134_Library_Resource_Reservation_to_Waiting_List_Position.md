# Library Resource Reservation to Waiting List Position

**Scenario ID:** L3-G-134  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, C, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

This scenario governs the complete institutional outcome in which a member requests a library resource that is not presently available for immediate issue, the request is validated under the applicable reservation policy, and the reservation is established in the waiting list with an authoritative queue position.

Reservation Management exists to provide equitable access to limited library resources. The L2 defines the reservation workflow as:

`Reservation Requested → Waiting List → Copy Available → Member Notified → Collection → Reservation Closed`.

Reservation validation may consider Active Membership, Existing Borrowings, Reservation Limits, Resource Availability and Duplicate Reservations. Waiting lists support Queue Position, Reservation Date, Priority, Notification Status and Reservation Expiry, with policies remaining configurable. fileciteturn179file0turn180file0

## 3. Business Outcome

A valid reservation request has been accepted and the member holds an authoritative waiting-list position for the requested resource.

The scenario does not make the resource available, allocate a physical copy, or close the reservation. Those are later outcomes in the reservation lifecycle.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A library member requests reservation of a resource that is unavailable for immediate issue.

### 4.2 End Boundary

The reservation is validated and reaches the authoritative Waiting condition with its queue position established.

### 4.3 In Scope

- Identify member.
- Confirm membership eligibility.
- Identify requested resource.
- Determine resource availability.
- Validate reservation limits.
- Check existing borrowings.
- Detect duplicate reservations.
- Establish reservation.
- Determine waiting-list position and priority.
- Record reservation date and applicable expiry.
- Preserve reservation history.

### 4.4 Out of Scope

- Membership activation.
- Issuing the resource.
- Physical-copy registration.
- Collection of the resource.
- Notification delivery.
- Reservation expiry/closure.
- Finance/payment processing.
- Technical queues, APIs, databases or UI.

## 5. Ownership and Participating Domains

### Library Management — G

Owns Reservation Management and the reservation lifecycle.

### Student Lifecycle — C

Provides authoritative student context where the member is a student. C does not own the reservation.

### Communication — H

Consumes reservation business outcomes for governed communication. H does not own reservation state.

Cross-domain participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

### Library Member

Requests reservation of the unavailable resource.

### Librarian / Circulation Desk Operator

Performs or authorises reservation processing under configured library policy.

### C Authority

Provides authoritative student context where applicable.

### H

Consumes the reservation outcome when communication is required.

## 7. Preconditions and Entry Criteria

1. The member can be identified.
2. The member has an Active Membership.
3. The requested resource can be identified.
4. The reservation policy is available.
5. The member has not exceeded the applicable reservation limit.
6. Duplicate-reservation conditions can be evaluated.
7. The resource is unavailable for immediate issue or otherwise eligible for reservation.

## 8. Trigger

A library member submits a reservation request for an unavailable library resource.

## 9. Main Business Flow

### Phase A — Validate the Member

1. Identify the requesting member.
2. Confirm Active Membership.
3. Establish applicable member/category context.
4. Evaluate existing borrowing information where required.
5. Evaluate configured reservation limits.

### Phase B — Validate the Reservation

6. Identify the requested resource.
7. Determine its current availability.
8. Confirm that reservation is applicable to the resource.
9. Check for an existing duplicate reservation.
10. Apply configured reservation validation rules.
11. Determine applicable reservation priority.

### Phase C — Establish Waiting Position

12. Register the reservation request.
13. Establish the reservation date.
14. Place the reservation in the Waiting condition.
15. Determine and record the authoritative Queue Position.
16. Record the applicable Priority.
17. Record Notification Status as applicable.
18. Record Reservation Expiry according to configured policy.

### Phase D — Complete

19. Preserve reservation history.
20. Make the Waiting reservation available to subsequent availability/notification processing.
21. End the scenario with the member holding an authoritative waiting-list position.

The scenario does not advance the reservation to Available, Collection or Closed.

## 10. Alternate Business Flows

### 10.1 Resource Becomes Available During Validation

If the resource becomes available before the reservation is established, the applicable configured reservation policy determines whether reservation remains appropriate. This scenario does not invent an automatic collection outcome.

### 10.2 Reservation Limit Exceeded

The reservation is rejected and no new waiting-list position is established.

### 10.3 Duplicate Reservation Exists

The duplicate request is rejected or otherwise handled according to configured duplicate-reservation policy. No second unsupported waiting position is created.

### 10.4 Existing Borrowing Affects Eligibility

Existing borrowings are evaluated according to configured reservation policy. The borrowing itself remains governed by Circulation Management.

### 10.5 Member Is Not Eligible

The request does not become an authoritative reservation.

## 11. Failure, Interruption and Business Recovery

### 11.1 Membership Status Cannot Be Established

Reservation establishment does not proceed until authoritative membership status can be evaluated.

### 11.2 Resource Availability Cannot Be Established

The reservation is not finalised until the authoritative availability condition is known.

### 11.3 Queue Position Cannot Be Established

The reservation does not become a completed Waiting outcome until the business waiting-list position can be established.

### 11.4 Reservation Validation Rejects the Request

No Waiting reservation is established; the rejection/decision is preserved as required by the business process.

No technical retry, queue implementation, rollback, or infrastructure mechanics are described.

## 12. Business Rules and Decision Constraints

Reservation validation may consider:

- Active Membership.
- Existing Borrowings.
- Reservation Limits.
- Resource Availability.
- Duplicate Reservations.

Waiting lists support:

- Queue Position.
- Reservation Date.
- Priority.
- Notification Status.
- Reservation Expiry.

Validation and queue-processing policies remain configurable. fileciteturn179file0turn180file0

The reservation lifecycle is:

`Requested → Waiting → Available → Collected → Closed`.

No local reservation state is invented. fileciteturn183file0

## 13. State and Lifecycle Impact

The scenario covers:

`Requested → Waiting`

The authoritative reservation lifecycle continues to:

`Waiting → Available → Collected → Closed`.

`Waiting` is the canonical lifecycle condition; “waiting-list position” is the business outcome established within that state.

No `Queued`, `Pending`, `Priority Assigned`, or `Notification Pending` lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

The L2 service model explicitly provides Reservation Services for Reserve Resource, Cancel Reservation, Allocate Reserved Resource and Query Reservation. fileciteturn183file0

The retrieved L2 event catalogue does not expose a dedicated reservation-request event name in the available source material. Therefore this draft does **not** invent an event such as `ReservationRequested`.

Business collaboration remains:

- C provides student context where applicable.
- H consumes the reservation outcome for communication.
- G remains the canonical owner of Reservation Management.

## 15. Business Authorization, Approval and Audit Obligations

Reservation processing follows the configured library authorization model. The G05 authorization model identifies Librarian and Circulation Desk among the operational roles and permits configurable authorization/delegation. fileciteturn173file0

The reservation request, eligibility decision, queue position, priority, expiry and resulting lifecycle outcome must be traceable.

No technical access control/IAM implementation is described.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Member identified.
- Active Membership validated.
- Resource identified.
- Resource availability evaluated.
- Reservation eligibility validated.
- Duplicate and limit checks completed.
- Reservation established.
- Waiting condition established.
- Queue Position recorded.
- Priority recorded.
- Reservation date/expiry information recorded.
- History preserved.

### Postconditions

- Reservation is in Waiting.
- Member has an authoritative waiting-list position.
- Subsequent availability processing can evaluate the reservation.
- The resource remains governed by its own Catalogue/Copy lifecycle.
- No collection or borrowing is created by this scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

The available L2 material does not expose a dedicated reservation-request event name. No event is invented.

### Assumptions

- Queue position is established by the configured waiting-list policy.
- Reservation priority remains configurable.
- Reservation expiry remains configurable.

### Deferred Questions

Confirm the canonical reservation event vocabulary upstream before Architecture Approval/Lock if the final event catalogue requires an event for reservation creation.
