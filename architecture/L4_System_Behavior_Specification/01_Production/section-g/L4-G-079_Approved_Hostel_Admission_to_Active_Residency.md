# Approved Hostel Admission to Active Residency

## 1. Specification Metadata

**Specification ID:** L4-G-079  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-079 — Approved Hostel Admission to Active Residency  
**Primary Business Owner:** Hostel Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-079; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall convert an approved hostel admission into active hostel residency through governed room allocation, arrival verification, check-in, and occupancy activation.

## 3. Required Business Context

- Hostel Admission is in Approved state.
- Canonical student identity/context is available.
- An eligible room may be allocated under applicable policy.
- Hostel Management owns Room Allocation, Check-In, and Hostel Occupancy.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-079-BH-001 | The system shall verify that the approved Hostel Admission remains eligible for residency activation. | L3-G-079 §§7, 12 |
| L4-G-079-BH-002 | The system shall identify and establish an eligible Room Allocation under applicable hostel allocation policy. | L3-G-079 §§9, 12 |
| L4-G-079-BH-003 | The system shall validate the arriving student against the approved admission and assigned room before occupancy activation. | L3-G-079 §§9, 12 |
| L4-G-079-BH-004 | The system shall complete required check-in verification before activating Hostel Occupancy. | L3-G-079 §§9, 12 |
| L4-G-079-BH-005 | When required activation conditions are satisfied, the system shall transition Hostel Occupancy to Active. | L3-G-079 §§3, 13, 16 |
| L4-G-079-BH-006 | The system shall maintain the relationship between Active Hostel Occupancy and exactly one active Room Allocation. | L3-G-079 §12 |
| L4-G-079-BH-007 | The system shall preserve attributable evidence of room allocation, check-in, and occupancy activation. | L3-G-079 §15 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-079-BH-008 | On establishment of the room allocation, the system shall make the canonical RoomAllocated business fact available through the G03 publisher. | L3-G-079 §14; L2-G03 |
| L4-G-079-BH-009 | On occupancy activation, the system shall make the canonical OccupancyActivated business fact available through the G03 publisher. | L3-G-079 §14; L2-G03 |

## 6. Failure / Rejection / Continuation

- If no eligible room is available, the system shall not activate occupancy and shall retain the business process for an applicable allocation or resolution path.
- If required identity or check-in evidence cannot be validated, the system shall prevent occupancy activation until the business deficiency is resolved.
- If the approved admission is no longer valid for activation, the system shall prevent occupancy activation.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of approved admission eligibility, room allocation, check-in verification, occupancy activation, and the applicable business authority/action.

## 8. Observable Completion Conditions

Completion occurs when: - an eligible room allocation exists; - required check-in verification is complete; - Hostel Occupancy is Active; - the occupancy references exactly one active room allocation; and - OccupancyActivated is made available through the canonical publisher.

## 9. Traceability & Provenance

**Parent:** L3-G-079 — Approved Hostel Admission to Active Residency  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
