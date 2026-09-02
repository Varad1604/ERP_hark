# Resident Leave Request to Verified Return

## 1. Specification Metadata

**Specification ID:** L4-G-082  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-082 — Resident Leave Request to Verified Return  
**Primary Business Owner:** Hostel Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-082; L2-G03 — Hostel Management Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Govern a resident leave request from submission through approved departure, verified return and closure while preserving the existing active Hostel Occupancy and Student Lifecycle ownership.

## 3. Required Business Context

- An active Hostel Occupancy exists.
- Canonical student identity/context is available.
- Applicable hostel leave policy and configured approval authority are available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-082-BH-001 | The system shall register the Leave Request against an active resident and establish `Applied`. | L3-G-082 §§7–9, 13 |
| L4-G-082-BH-002 | The system shall evaluate the request under applicable hostel leave policy and progress it through the configured review decision. | L3-G-082 §§6, 9, 12 |
| L4-G-082-BH-003 | When approved, the system shall establish `Approved`; an unapproved request shall not create an authorised leave outcome. | L3-G-082 §§9–13 |
| L4-G-082-BH-004 | When the resident departs under approved leave, the system shall establish `Resident Left` while retaining the active Hostel Occupancy. | L3-G-082 §§3, 9, 13 |
| L4-G-082-BH-005 | On return, the system shall verify the resident against the applicable Leave Request and establish `Resident Returned`. | L3-G-082 §§9, 13, 15 |
| L4-G-082-BH-006 | After verified return, the system shall establish `Closed` and preserve the complete leave history. | L3-G-082 §§3, 13, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-082-BH-007 | G03 Hostel Management remains the canonical publisher of material leave and resident-movement facts; the system shall not invent a leave event identifier because the Locked parent records the exact event names as unresolved. | L3-G-082 §§14, 17 |
| L4-G-082-BH-008 | Student Lifecycle provides canonical resident identity/context and Notifications may consume leave outcomes without acquiring ownership of Leave Request or Hostel Occupancy. | L3-G-082 §§5, 14 |

## 6. Failure / Rejection / Continuation

- An unapproved request shall not authorize resident departure.
- If expected return cannot be verified, the Leave Request shall not be falsely closed; the applicable movement/curfew/discipline process governs the unresolved condition.
- Temporary leave shall not close Hostel Occupancy.

## 7. Audit & Evidence Behavior

Preserve request, review, authority, decision, approved period, departure, return verification and closure history.

## 8. Observable Completion Conditions

Completion requires the applicable leave decision, resident movement where approved, verified return, `Closed` Leave Request, and continued active Hostel Occupancy.

## 9. Traceability & Provenance

**Parent:** L3-G-082 — Resident Leave Request to Verified Return  
**Primary verification source:** L3-G-082 §§3, 5, 9, 11–17 and authoritative L2-G03 architecture.

## 10. Open Issues / Deferred Design

Exact canonical G03 leave-event identifiers remain unresolved upstream in L3-G-082 §17. Institution-specific approval sequencing remains configuration governed. Technical implementation is deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
