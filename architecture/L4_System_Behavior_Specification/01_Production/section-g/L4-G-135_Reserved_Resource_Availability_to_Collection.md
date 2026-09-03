# Reserved Resource Availability to Collection

## 1. Specification Metadata

**Specification ID:** L4-G-135  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-135 — Reserved Resource Availability to Collection  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-135; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern availability of a reserved resource through selection of the entitled Waiting reservation, collection readiness, governed notification hand-off, and collection without duplicating the later circulation issue outcome.

## 3. Required Business Context

- One or more valid Waiting reservations exist for the resource.
- Resource availability is authoritative.
- Member/reservation eligibility, queue position, priority, collection and expiry policy can be evaluated.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-135-BH-001 | Identify Waiting reservations for the newly available resource. | L3-G-135 §§7,9 |
| L4-G-135-BH-002 | Select the next eligible reservation using configured queue-position and priority policy. | L3-G-135 §§9–10,12 |
| L4-G-135-BH-003 | Establish the selected reservation in the applicable collection-ready condition and record collection-window/expiry information. | L3-G-135 §§9,12–13 |
| L4-G-135-BH-004 | Make the notification requirement/status available to H while preserving H ownership of notification delivery. | L3-G-135 §§5,9,14 |
| L4-G-135-BH-005 | On valid collection, record the reservation as `Collected`, preserve history, and expose the outcome to subsequent circulation without performing the issue transaction. | L3-G-135 §§3,9,13,16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-135-BH-006 | The retrieved L2 event catalogue does not expose a dedicated reservation-availability or collection event. This specification shall not invent `ReservationAvailable`, `ReservationCollected`, or an equivalent event. C may provide student context; H owns notification delivery; G remains owner of Reservation. | L3-G-135 §§5,14,17 |

## 6. Failure / Rejection / Continuation

- Expired, cancelled or ineligible reservations shall not be selected for collection.
- Collection shall not proceed if member/reservation eligibility cannot be established.
- Failure to collect within configured policy remains subject to the separate reservation-expiry workflow.
- Actual borrowing/issue remains a separate Circulation outcome.

## 7. Audit & Evidence Behavior

Preserve resource availability, selected reservation, queue/priority basis, eligibility, notification status, collection outcome and reservation history.

## 8. Observable Completion Conditions

Completion occurs when the entitled reservation reaches `Collected` with traceable selection and collection history, while subsequent borrowing remains separate.

## 9. Traceability & Provenance

**Parent:** L3-G-135 — Reserved Resource Availability to Collection  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Canonical reservation availability/collection event vocabulary remains unresolved upstream. Queue/priority and collection/expiry windows remain configuration governed. Technical notification delivery, queues, APIs, database persistence schemas and UI are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
