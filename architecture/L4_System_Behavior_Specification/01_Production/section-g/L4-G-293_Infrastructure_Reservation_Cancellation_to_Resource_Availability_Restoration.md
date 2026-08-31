# Infrastructure Reservation Cancellation to Resource Availability Restoration

## 1. Specification Metadata
**Specification ID:** L4-G-293  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-293  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-G-293_Infrastructure_Reservation_Cancellation_to_Resource_Availability_Restoration.md`; L2-G10 — Infrastructure Management Workflows; L2-A04; L2-A06; L2-A07  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall cancel a G10 Reservation through its canonical cancellation branch, release its reservation constraint, preserve history, and recalculate resource availability under G10 policy.

## 3. Required Business Context
The Reservation and resource shall exist; the Reservation shall be in `Requested`, `Confirmed` or `Active`; cancellation shall be authorized; authoritative availability policy shall be available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-293-BH-001 | The system shall reject cancellation when the Reservation/resource is invalid, the Reservation is outside a cancellable source-defined position, or authorization fails. | L3-G-293 §§7,9,11–12 |
| L4-G-293-BH-002 | On valid cancellation, the system shall transition the Reservation from `Requested`, `Confirmed` or `Active` to `Cancelled`. | L3-G-293 §§9,12–13 |
| L4-G-293-BH-003 | The system shall release the reservation constraint without deleting Reservation history. | L3-G-293 §§2–4,9,12 |
| L4-G-293-BH-004 | The system shall recalculate the resource's current Availability Profile and shall mark it `Available` only when no other authoritative restriction applies. | L3-G-293 §§9–12 |
| L4-G-293-BH-005 | A cancelled Reservation may proceed to `Archived` for historical preservation; `Closed` shall not be introduced as an additional lifecycle state. | L3-G-293 §§9,12–13 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-293-BH-006 | On successful cancellation, the system shall publish `ReservationCancelled` from G10. | L3-G-293 §§9,12,14 |

## 6. Failure / Rejection / Continuation
Unauthorized or invalid cancellation shall preserve the current Reservation and resource conditions. Cancellation alone shall not falsely restore availability when another restriction remains.

## 7. Audit & Evidence Behavior
Evidence shall preserve Reservation/resource identity, prior state, cancellation reason/authority, `Cancelled` result, release/availability result and event publication.

## 8. Observable Completion Conditions
The Reservation is `Cancelled`, no longer reserves the resource, history is preserved, resource availability reflects G10 policy and `ReservationCancelled` is published.

## 9. Traceability & Provenance
**Parent:** L3-G-293 — Infrastructure Reservation Cancellation to Resource Availability Restoration  
**Primary verification source:** L2-G10 — Infrastructure Management Workflows

## 10. Open Issues / Deferred Design
The parent defines `Cancelled → Archived` as the governed cancellation branch; no additional state is introduced.
