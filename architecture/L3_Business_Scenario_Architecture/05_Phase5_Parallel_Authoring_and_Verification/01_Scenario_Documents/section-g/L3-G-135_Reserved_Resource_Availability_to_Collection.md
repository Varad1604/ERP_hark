# Reserved Resource Availability to Collection

**Scenario ID:** L3-G-135  
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

This scenario governs the complete institutional outcome in which a reserved resource becomes available, the applicable reservation is identified according to configured waiting-list policy, the member is notified, and the reserved resource is collected.

The L2 reservation workflow is:

`Reservation Requested → Waiting List → Copy Available → Member Notified → Collection → Reservation Closed`.

Waiting-list information includes Queue Position, Reservation Date, Priority, Notification Status and Reservation Expiry. Reservation status values include Requested, Waiting, Ready for Collection, Collected, Expired and Cancelled, with status values remaining configurable. fileciteturn179file0turn180file0

## 3. Business Outcome

A reserved resource becomes available, the applicable reservation reaches the collection stage for the entitled member, and the member successfully collects the reserved resource under the configured reservation and circulation policy.

The scenario ends with collection. Any subsequent borrowing/issue outcome follows the Circulation workflow and is not duplicated here.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A reserved resource becomes available for allocation to a waiting reservation.

### 4.2 End Boundary

The entitled reservation has been collected and the reservation reaches the applicable Collected condition.

### 4.3 In Scope

- Detect resource availability as a business fact.
- Identify applicable waiting reservation.
- Apply queue position/priority policy.
- Establish reservation readiness for collection.
- Notify the entitled member through H.
- Record notification status.
- Record collection.
- Update reservation lifecycle/history.
- Hand off to subsequent circulation where applicable.

### 4.4 Out of Scope

- Creation of the reservation.
- Membership activation.
- Library Copy registration.
- Fine calculation.
- Payment.
- Actual circulation issue transaction.
- Notification delivery implementation.
- Technical queue processing.

## 5. Ownership and Participating Domains

### Library Management — G

Owns Reservation Management and determines reservation allocation/collection business outcomes.

### Student Lifecycle — C

Provides authoritative student context where applicable.

### Communication — H

Owns notification delivery. G establishes the business notification requirement; H delivers it.

No domain assumes ownership of Reservation.

## 6. Business Actors, Roles and Responsibilities

### Librarian / Circulation Desk Operator

Processes reservation allocation and collection according to configured policy.

### Reserved Member

Receives notification and collects the resource within the applicable collection policy.

### C Authority

Provides student context where applicable.

### H

Delivers reservation-availability communication.

## 7. Preconditions and Entry Criteria

1. A valid Waiting reservation exists.
2. The requested resource/copy becomes available.
3. Reservation eligibility remains valid.
4. Queue position/priority can be evaluated.
5. The member remains eligible under the configured policy.
6. The reservation has not expired or been cancelled.

## 8. Trigger

A resource with one or more waiting reservations becomes available.

## 9. Main Business Flow

### Phase A — Establish Availability

1. Identify the resource that has become available.
2. Identify Waiting reservations for the resource.
3. Evaluate queue position and configured reservation priority.
4. Select the reservation entitled to the available resource.
5. Confirm that the selected reservation remains eligible.

### Phase B — Prepare for Collection

6. Move the selected reservation into the configured collection-ready condition.
7. Record the resource/copy availability against the reservation.
8. Establish the applicable collection window/expiry according to policy.
9. Record the Notification Status.
10. Provide the availability outcome to H for member notification.

### Phase C — Collection

11. Member presents for collection.
12. Verify the reservation and member according to configured library authority.
13. Confirm that the reservation remains valid at collection time.
14. Record collection of the reserved resource.
15. Move the reservation to Collected.
16. Preserve reservation history.

### Phase D — Complete

17. Make the collected outcome available to subsequent circulation processing.
18. End the scenario.

The actual borrowing/issue transaction is a separate circulation outcome.

## 10. Alternate Business Flows

### 10.1 Multiple Members Are Waiting

The next eligible reservation is selected according to configured queue position and priority policy. No local prioritisation rule is invented.

### 10.2 Selected Reservation Has Expired

The expired reservation is not collected. The next applicable reservation is considered according to configured policy.

### 10.3 Member Is No Longer Eligible

The reservation is not collected and is handled under the applicable reservation policy.

### 10.4 Member Does Not Collect Within the Applicable Period

Reservation expiry is handled by the Reservation Expiry workflow. This scenario does not invent an expiry duration.

### 10.5 Resource Is No Longer Available

The reservation remains subject to the authoritative reservation lifecycle and availability conditions; collection does not proceed.

### 10.6 Collection Completed but Borrowing Is Separate

Collection establishes the reservation outcome. The subsequent issue/borrowing transaction follows the circulation workflow.

## 11. Failure, Interruption and Business Recovery

### 11.1 Waiting Reservation Cannot Be Identified

No collection is established until the applicable reservation can be determined.

### 11.2 Eligibility Cannot Be Verified

The resource is not treated as collected until the applicable member/reservation conditions are established.

### 11.3 Notification Delivery Is Unsuccessful

The business notification requirement remains with H. G does not implement technical notification retries.

### 11.4 Collection Is Rejected

The reservation does not move to Collected; the applicable reservation condition remains authoritative.

### 11.5 Resource Is Claimed by Another Valid Outcome

The reservation allocation is resolved according to configured reservation/circulation policy. No competing local rule is invented.

## 12. Business Rules and Decision Constraints

Reservation validation may consider Active Membership, Existing Borrowings, Reservation Limits, Resource Availability and Duplicate Reservations.

Waiting lists support Queue Position, Reservation Date, Priority, Notification Status and Reservation Expiry. Queue-processing policies remain configurable. fileciteturn179file0turn180file0

Reservation statuses include Requested, Waiting, Ready for Collection, Collected, Expired and Cancelled. fileciteturn180file0

The canonical reservation lifecycle is:

`Requested → Waiting → Available → Collected → Closed`.

No local state or expiry rule is invented. fileciteturn183file0

## 13. State and Lifecycle Impact

The business flow advances the reservation from the Waiting condition through availability/readiness to Collected.

The authoritative lifecycle is:

`Requested → Waiting → Available → Collected → Closed`.

The L2 also defines `Ready for Collection` as a supported reservation status. This scenario may therefore use that configured operational status without treating it as a separate canonical lifecycle aggregate state.

The scenario does not close the reservation after collection unless the authoritative lifecycle transition requires the subsequent Closed step; it ends at the frozen business outcome of collection.

## 14. Business Events and Cross-Domain Collaboration

The available L2 service model explicitly supports allocation of a reserved resource through Reservation Services. fileciteturn183file0

The retrieved event catalogue does not expose a dedicated reservation-availability/collection event name. Therefore this draft does not invent `ReservationAvailable`, `ReservationCollected`, or similar event names.

Business collaboration:

- C supplies student context where applicable.
- H delivers notification after G establishes the business notification outcome.
- G owns the reservation and collection decision.
- Circulation may subsequently consume the collection outcome for issue processing.

## 15. Business Authorization, Approval and Audit Obligations

Reservation allocation and collection follow configured library authorization.

The G05 authorization model identifies Librarian and Circulation Desk as operational roles and supports delegated authority. fileciteturn173file0

The selected reservation, queue/priority basis, notification status, collection outcome and reservation history must be auditable.

No technical authorization or notification implementation is described.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Resource availability established.
- Waiting reservations evaluated.
- Correct reservation selected.
- Eligibility confirmed.
- Reservation made ready for collection.
- Notification outcome established.
- Member notified through H where applicable.
- Collection verified.
- Reservation reaches Collected.
- History preserved.
- Subsequent circulation hand-off made available.

### Postconditions

- The selected reservation is Collected.
- The reserved resource has been collected by the entitled member.
- Reservation history is preserved.
- Subsequent borrowing/issue processing may proceed separately.
- Other waiting reservations remain governed by the reservation policy.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

The retrieved L2 does not expose a dedicated event name for reservation availability or collection. No event is invented.

### Assumptions

- Queue position and priority determine the entitled reservation.
- Collection/expiry windows remain configurable.
- The actual circulation issue remains a separate business scenario.

### Deferred Questions

Confirm the canonical reservation event vocabulary upstream if event publication is mandatory before L3 approval/lock.
