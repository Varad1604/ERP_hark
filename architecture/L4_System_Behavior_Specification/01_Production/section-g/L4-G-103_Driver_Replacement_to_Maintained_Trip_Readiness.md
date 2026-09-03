# Driver Replacement to Maintained Trip Readiness

## 1. Specification Metadata

**Specification ID:** L4-G-103  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-103 — Driver Replacement to Maintained Trip Readiness  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-103; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern replacement of the driver assigned to a planned Trip while preserving substitution history and People & Workforce ownership of driver/personnel records.

## 3. Required Business Context

- An existing Trip has an operational Driver Assignment.
- A supported business basis for replacement exists.
- Authoritative candidate driver/personnel context is available from People & Workforce.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-103-BH-001 | Identify the affected Trip, current Driver Assignment, and replacement basis. | L3-G-103 §§7–9 |
| L4-G-103-BH-002 | Use authoritative People & Workforce context to evaluate replacement candidates for applicable eligibility and availability. | L3-G-103 §§5, 9, 12 |
| L4-G-103-BH-003 | Establish the replacement operational Driver Assignment only when the selected candidate satisfies the applicable transport conditions. | L3-G-103 §§9, 12, 16 |
| L4-G-103-BH-004 | Preserve the outgoing assignment and substitution history when the replacement becomes effective. | L3-G-103 §§3, 9, 12, 15–16 |
| L4-G-103-BH-005 | Confirm the Trip's driver-readiness condition only after the replacement assignment is effective. | L3-G-103 §§3, 9, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-103-BH-006 | People & Workforce remains the authoritative owner of driver/personnel records; Transport Management owns only the operational replacement assignment and substitution history. | L3-G-103 §§5, 12, 14, 16 |
| L4-G-103-BH-007 | The exact canonical Driver Replacement event identifier is unresolved upstream; this specification shall not invent one. | L3-G-103 §§14, 17 |

## 6. Failure / Rejection / Continuation

- An unavailable or ineligible candidate shall not become the replacement assignment.
- If no suitable replacement exists, driver-readiness shall remain unresolved rather than being treated as maintained.
- Workforce master-data correction remains with People & Workforce.

## 7. Audit & Evidence Behavior

Preserve replacement basis, outgoing assignment, authoritative candidate context considered, replacement decision, effective assignment, substitution history, and readiness evidence.

## 8. Observable Completion Conditions

A valid replacement Driver Assignment is effective, the prior assignment remains historically traceable, and the Trip's driver-readiness condition is maintained.

## 9. Traceability & Provenance

**Parent:** L3-G-103 — Driver Replacement to Maintained Trip Readiness  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Exact Driver Replacement lifecycle/event vocabulary remains unresolved upstream and is deliberately not invented. Technical implementation is deferred to later architecture levels.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
