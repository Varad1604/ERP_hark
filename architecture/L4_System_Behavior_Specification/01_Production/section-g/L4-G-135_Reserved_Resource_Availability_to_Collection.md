# Reserved Resource Availability to Collection

## 1. Specification Metadata
**Specification ID:** L4-G-135  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Library Management  
**Realizes Scenario:** L3-G-135 — Reserved Resource Availability to Collection  
**Primary Business Owner:** Library Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-G-135 — Reserved Resource Availability to Collection

## 2. Behavioral Objective & Boundary
SutraERP shall identify an eligible Waiting reservation when a reserved resource becomes available, establish collection readiness, support governed notification and record collection.

**Start:** Reserved resource becomes available.  
**End:** Entitled reservation reaches Collected, or collection is not completed under the applicable reservation policy.

## 3. Required Business Context
- Waiting reservation exists.
- Resource becomes available.
- Member and reservation remain eligible.
- Queue/priority and collection policy are authoritative.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-135-BH-001 | Identify Waiting reservations for the available resource. | L3-G-135 §§7,9 |
| L4-G-135-BH-002 | Select the next eligible reservation using configured queue/priority policy. | L3-G-135 §§9,10 |
| L4-G-135-BH-003 | Establish the selected reservation as ready for collection and record applicable collection-window information. | L3-G-135 §§9,12 |
| L4-G-135-BH-004 | Make the notification requirement available to H without transferring notification ownership. | L3-G-135 §§5,9,14 |
| L4-G-135-BH-005 | On valid collection, record the reservation as Collected and preserve reservation history. | L3-G-135 §§9,16 |

## 5. Cross-Domain & Event Behavior
H owns notification delivery; C provides student context where applicable. The available L2 does not establish a dedicated reservation-availability/collection event, so none is invented.

## 6. Failure / Rejection / Continuation
- Expired or ineligible reservations shall not be selected for collection.
- If eligibility cannot be verified, collection shall not be established.
- If collection does not occur within configured policy, the reservation remains subject to the expiry workflow.
- Subsequent allocation follows the authoritative reservation process.

## 7. Audit & Evidence Behavior
Preserve selected reservation, queue/priority basis, resource availability, notification status, collection decision and reservation history.

## 8. Observable Completion Conditions
Completion occurs when the entitled reservation is Collected with history preserved, or the applicable non-collection outcome is recorded.

## 9. Traceability & Provenance
**Parent:** L3-G-135 — Reserved Resource Availability to Collection  
**Primary verification source:** L3-G-135; locked L2 Reservation Management workflow

## 10. Open Issues / Deferred Design
Exact collection windows and event vocabulary remain governed upstream. Notification transport and technical implementation remain outside L4.