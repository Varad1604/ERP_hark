# Library Resource Reservation to Waiting List Position

## 1. Specification Metadata

**Specification ID:** L4-G-134  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-134 — Library Resource Reservation to Waiting List Position  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-134; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall validate reservation of a resource unavailable for immediate issue and establish an authoritative `Waiting` reservation with queue position under configured Library Management policy.

## 3. Required Business Context

- The member has an Active Library Membership.
- The resource and its availability are authoritative.
- Existing borrowings, reservation limits, duplicate-reservation conditions, queue policy and priority can be evaluated.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-134-BH-001 | Validate the requesting member and Active Membership. | L3-G-134 §§7,9 |
| L4-G-134-BH-002 | Evaluate resource availability, existing borrowings, reservation limits, duplicate reservations and configured eligibility rules. | L3-G-134 §§7,9,12 |
| L4-G-134-BH-003 | Establish the reservation through `Requested → Waiting` when reservation criteria are satisfied. | L3-G-134 §§3,9,13 |
| L4-G-134-BH-004 | Establish the authoritative Queue Position and applicable Priority under configured waiting-list policy. | L3-G-134 §§3,9,12 |
| L4-G-134-BH-005 | Preserve reservation date, notification status where applicable, expiry information and reservation history. | L3-G-134 §§9,12,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-134-BH-006 | The retrieved L2 event catalogue does not expose a dedicated reservation-request event; this specification shall not invent `ReservationRequested` or an equivalent event. C may provide student context and H may consume the reservation outcome without acquiring Reservation ownership. | L3-G-134 §§5,14,17 |

## 6. Failure / Rejection / Continuation

- Ineligible membership or resource conditions shall prevent reservation establishment.
- Exceeded reservation limits shall prevent a new Waiting position.
- A duplicate reservation shall not create a second unsupported Waiting position.
- If queue position cannot be established, the reservation shall not be treated as a completed Waiting outcome.
- No `Queued`, `Pending`, or `Priority Assigned` lifecycle state shall be invented.

## 7. Audit & Evidence Behavior

Preserve member/resource context, eligibility decision, queue position, priority, reservation date, expiry and reservation history.

## 8. Observable Completion Conditions

Completion occurs when a valid reservation is in `Waiting` with an authoritative queue position and history preserved, or a governed rejection is recorded.

## 9. Traceability & Provenance

**Parent:** L3-G-134 — Library Resource Reservation to Waiting List Position  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Canonical reservation-request event vocabulary remains unresolved upstream. Queue position, priority and expiry remain configuration governed. Technical queue implementation, APIs, database persistence schemas, UI screens and infrastructure are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
