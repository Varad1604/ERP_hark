# Reservation Expiry to Reservation Closure

## 1. Specification Metadata

**Specification ID:** L4-G-136  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-136 — Reservation Expiry to Reservation Closure  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-136; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a reservation that reaches its configured expiry condition without collection through the supported `Expired` status and authoritative reservation closure, without inventing expiry duration or successor-allocation rules.

## 3. Required Business Context

- A valid reservation exists and has not already been collected, cancelled or closed.
- The configured expiry condition and current reservation condition can be established.
- Reservation lifecycle/status policy is authoritative.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-136-BH-001 | Identify the reservation and confirm its current authoritative condition. | L3-G-136 §§7,9 |
| L4-G-136-BH-002 | Establish that the configured expiry condition has been reached without collection. | L3-G-136 §§2,8–9 |
| L4-G-136-BH-003 | Record the supported `Expired` status where applicable and establish the terminal `Closed` reservation outcome. | L3-G-136 §§3,9,12–13,16 |
| L4-G-136-BH-004 | Make the expired reservation no longer eligible for ordinary collection without automatically creating or selecting a successor reservation. | L3-G-136 §§3,9–10,16 |
| L4-G-136-BH-005 | Preserve queue position, reservation date, priority, notification status, expiry information and history. | L3-G-136 §§9,12,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-136-BH-006 | The retrieved L2 event material does not expose a dedicated reservation-expiry or reservation-closure event. This specification shall not invent `ReservationExpired`, `ReservationClosed`, or an equivalent event. C may provide student context and H may consume the expiry/closure outcome; G remains owner of Reservation. | L3-G-136 §§5,14,17 |

## 6. Failure / Rejection / Continuation

- If expiry cannot be established, the reservation remains in its current authoritative condition.
- If collection status cannot be established, expiry closure shall not be finalised.
- Cancelled or collected reservations shall not be processed as newly expired.
- Subsequent allocation follows the separate Reservation workflow under configured queue policy.

## 7. Audit & Evidence Behavior

Preserve expiry basis, collection determination, status change, closure outcome and material reservation history.

## 8. Observable Completion Conditions

Completion occurs when the reservation has the supported `Expired` status as applicable, reaches `Closed`, is no longer an active collection entitlement, and history remains traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-136 — Reservation Expiry to Reservation Closure  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Canonical reservation expiry/closure event vocabulary remains unresolved upstream. Expiry conditions and subsequent queue processing remain configuration governed. Technical timers, schedulers, queue implementation, APIs, database persistence schemas and UI are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
