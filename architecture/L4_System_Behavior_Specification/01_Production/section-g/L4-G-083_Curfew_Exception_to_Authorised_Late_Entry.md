# L4-G-083 — Curfew Exception to Authorised Late Entry

## 1. Specification Metadata

**Specification ID:** L4-G-083  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Hostel Management  
**Realizes Scenario:** L3-G-083 — Curfew Exception to Authorised Late Entry  
**Primary Business Owner:** Hostel Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-G-083 — Curfew Exception to Authorised Late Entry  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a resident's curfew exception request and authorised late entry without changing Student Lifecycle ownership.

**Start:** A resident requires an exception to the applicable curfew.  
**End:** The exception is authorised or rejected and the resulting late-entry outcome is recorded.

## 3. Required Business Context

- An active hostel residency exists.
- Applicable curfew policy is authoritative.
- An authorised hostel authority is available for exception decisioning.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-083-BH-001 | The system shall register the curfew exception request against the applicable resident context. | L3-G-083 §§2, 8, 9 |
| L4-G-083-BH-002 | The system shall evaluate the request against applicable curfew exception criteria. | L3-G-083 §§3, 7, 12 |
| L4-G-083-BH-003 | The system shall record the authorised or rejected exception outcome. | L3-G-083 §§3, 9, 13 |
| L4-G-083-BH-004 | When the exception is authorised, the system shall make the resident eligible for the governed late-entry outcome. | L3-G-083 §§3, 9 |
| L4-G-083-BH-005 | The system shall preserve the authority and decision evidence for the exception outcome. | L3-G-083 §15 |

## 5. Cross-Domain & Event Behavior

No specific event name is asserted by the supplied L3 parent. The system shall make the authorised curfew exception outcome available to authorised hostel consumers without transferring ownership.

## 6. Failure / Rejection / Continuation

- If the request does not satisfy exception criteria, the system shall record the governed rejection and shall not grant late-entry authorisation.
- If required decision authority is unavailable, the system shall retain the request unresolved rather than granting an unauthorised exception.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of the request, curfew basis, decision authority, decision outcome, and any authorised late-entry result.

## 8. Observable Completion Conditions

Completion occurs when the exception reaches an authorised or rejected outcome and the corresponding resident eligibility position is recorded.

## 9. Traceability & Provenance

**Parent:** L3-G-083 — Curfew Exception to Authorised Late Entry  
**Primary verification source:** Applicable locked L2-G03 — Hostel Management Workflows

## 10. Open Issues / Deferred Design

Technical notification, persistence, authentication, and implementation mechanics remain outside L4.