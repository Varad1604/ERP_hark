# Reservation Expiry to Reservation Closure

## 1. Specification Metadata
**Specification ID:** L4-G-136  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Library Management  
**Realizes Scenario:** L3-G-136 — Reservation Expiry to Reservation Closure  
**Primary Business Owner:** Library Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-G-136 — Reservation Expiry to Reservation Closure

## 2. Behavioral Objective & Boundary
SutraERP shall identify a reservation reaching its configured expiry condition, record expiry, establish the authoritative closed outcome and preserve reservation history.

**Start:** Reservation reaches configured expiry without collection.  
**End:** Reservation is no longer actionable for collection and reaches the authoritative closed outcome.

## 3. Required Business Context
- Valid reservation exists and has not been collected/cancelled/closed.
- Configured expiry condition is authoritative and has been reached.
- Reservation lifecycle and status vocabulary are available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-136-BH-001 | Confirm the reservation and its current lifecycle condition before expiry processing. | L3-G-136 §§7,9 |
| L4-G-136-BH-002 | Establish that the configured expiry condition has been reached without collection. | L3-G-136 §§2,9 |
| L4-G-136-BH-003 | Record the reservation as Expired where that supported status applies. | L3-G-136 §§9,12,13 |
| L4-G-136-BH-004 | Establish the authoritative reservation closure outcome without inventing a successor-reservation rule. | L3-G-136 §§3,9,10 |
| L4-G-136-BH-005 | Preserve queue position, priority, expiry information and reservation history. | L3-G-136 §§9,15 |

## 5. Cross-Domain & Event Behavior
No dedicated canonical reservation-expiry/closure event is established by the available source. H may consume the outcome for communication; G remains owner.

## 6. Failure / Rejection / Continuation
- If expiry cannot be established, the reservation remains in its current authoritative condition.
- If collection status cannot be established, expiry closure shall not be finalised.
- Cancelled or collected reservations shall not be processed as newly expired.
- Subsequent allocation to another reservation follows the separate reservation workflow.

## 7. Audit & Evidence Behavior
Preserve expiry condition, collection determination, status change, closure outcome and material reservation history.

## 8. Observable Completion Conditions
Completion occurs when the reservation has reached the supported Expired condition and authoritative closure outcome, with history preserved.

## 9. Traceability & Provenance
**Parent:** L3-G-136 — Reservation Expiry to Reservation Closure  
**Primary verification source:** L3-G-136; locked L2 Reservation Management workflow

## 10. Open Issues / Deferred Design
Expiry duration, successor selection and dedicated event vocabulary remain policy/upstream concerns. Technical scheduling and queue mechanisms remain outside L4.