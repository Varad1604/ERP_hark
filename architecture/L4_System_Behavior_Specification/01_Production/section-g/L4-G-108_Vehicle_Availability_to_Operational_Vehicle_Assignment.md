# L4-G-108 — Vehicle Availability to Operational Vehicle Assignment

## 1. Specification Metadata
**Specification ID:** L4-G-108  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-108 — Vehicle Availability to Operational Vehicle Assignment  
**Primary Business Owner:** Transport Management  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01

## 2. Behavioral Objective & Boundary
SutraERP shall establish an operational vehicle assignment for a Trip from an available vehicle context without transferring vehicle ownership to Transport Management.

**Start:** A Trip requires an operational vehicle assignment.  
**End:** An eligible vehicle is operationally assigned, or a supported non-assignment outcome is recorded.

## 3. Required Business Context
- Trip and route context are authoritative.
- Vehicle availability can be evaluated.
- Vehicle asset ownership remains outside G04.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-108-BH-001 | Identify the Trip requirement for operational vehicle assignment. | L3-G-108 |
| L4-G-108-BH-002 | Evaluate available vehicles against applicable operational eligibility conditions. | L3-G-108 |
| L4-G-108-BH-003 | Establish the authorised operational vehicle assignment. | L3-G-108 |
| L4-G-108-BH-004 | Prevent assignment of a vehicle that fails applicable operational conditions. | L3-G-108 |
| L4-G-108-BH-005 | Preserve assignment history and the vehicle context used for the decision. | L3-G-108 |

## 5. Cross-Domain & Event Behavior
G04 owns operational assignment only; vehicle ownership and asset lifecycle remain outside G04. No unsupported event name is invented.

## 6. Failure / Rejection / Continuation
- No eligible vehicle means the Trip cannot satisfy vehicle-dependent readiness conditions.
- An unavailable or ineligible vehicle shall not be assigned.
- Vehicle maintenance or ownership decisions remain outside this scenario.

## 7. Audit & Evidence Behavior
Preserve vehicle availability assessment, eligibility basis, authority, assignment outcome and historical assignment changes.

## 8. Observable Completion Conditions
An eligible vehicle is operationally assigned to the Trip and the assignment is traceable, without creating vehicle ownership in G04.

## 9. Traceability & Provenance
**Parent:** L3-G-108 — Vehicle Availability to Operational Vehicle Assignment

## 10. Open Issues / Deferred Design
Technical vehicle registry integration, persistence and interfaces remain outside L4.