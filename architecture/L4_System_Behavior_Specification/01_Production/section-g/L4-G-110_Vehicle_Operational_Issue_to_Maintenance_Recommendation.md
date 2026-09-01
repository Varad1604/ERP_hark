# L4-G-110 — Vehicle Operational Issue to Maintenance Recommendation

## 1. Specification Metadata
**Specification ID:** L4-G-110  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-110 — Vehicle Operational Issue to Maintenance Recommendation  
**Primary Business Owner:** Transport Management  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01

## 2. Behavioral Objective & Boundary
SutraERP shall record a vehicle operational issue and establish an authorised maintenance recommendation without transferring maintenance or asset ownership to Transport Management.

**Start:** An operational vehicle issue is identified.  
**End:** A governed maintenance recommendation is recorded and handed to the owning maintenance process.

## 3. Required Business Context
- Vehicle context is identifiable.
- The issue is relevant to transport operations.
- Vehicle maintenance ownership remains with its canonical owner.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-110-BH-001 | Record the operational vehicle issue and affected operational context. | L3-G-110 |
| L4-G-110-BH-002 | Assess the issue against applicable operational criteria. | L3-G-110 |
| L4-G-110-BH-003 | Determine and record the governed maintenance recommendation where supported. | L3-G-110 |
| L4-G-110-BH-004 | Prevent Transport Management from assuming ownership of maintenance execution. | L3-G-110 |
| L4-G-110-BH-005 | Preserve issue, assessment and recommendation history. | L3-G-110 |

## 5. Cross-Domain & Event Behavior
The maintenance recommendation shall be made available to the canonical maintenance owner. Transport Management retains operational context; maintenance execution remains externally owned. No unsupported event name is invented.

## 6. Failure / Rejection / Continuation
- Insufficient issue information shall prevent a final recommendation.
- An issue outside maintenance scope shall be recorded with the appropriate governed outcome.
- Maintenance execution shall not be performed as part of this scenario.

## 7. Audit & Evidence Behavior
Preserve issue observation, assessment basis, recommendation, authority and hand-off evidence.

## 8. Observable Completion Conditions
The operational issue is recorded, an applicable recommendation is established or a supported non-recommendation outcome is recorded, and the owning maintenance process can receive the business outcome.

## 9. Traceability & Provenance
**Parent:** L3-G-110 — Vehicle Operational Issue to Maintenance Recommendation

## 10. Open Issues / Deferred Design
Technical maintenance-system integration and implementation mechanics remain outside L4.