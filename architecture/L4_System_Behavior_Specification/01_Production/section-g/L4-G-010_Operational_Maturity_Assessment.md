# Operational Maturity Assessment

## 1. Specification Metadata
**Specification ID:** L4-G-010  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations Governance  
**Realizes Scenario:** L3-G-010  
**Primary Business Owner:** Campus Operations Governance  
**Version:** 1.1.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-G-010; L2-G00 Governance Resolution and Operational Maturity Amendment

## 2. Behavioral Objective & Boundary
Produce a reproducible G00 maturity result from a governed maturity-framework version and source-owned evidence.

## 3. Required Business Context
An active maturity framework version exists; source facts remain source-owned; Governance Coordinator, Campus Operations Manager and Executive Reviewer perform their defined roles.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior |
|---|---|
| L4-G-010-BH-001 | Create the assessment in `Planned` and record the framework version. |
| L4-G-010-BH-002 | Progress to `Evidence Collection` and reference required evidence from canonical owners. |
| L4-G-010-BH-003 | Evaluate only according to the active framework's criteria/rules and record the resulting classification/level. |
| L4-G-010-BH-004 | Publish `OperationalMaturityEvaluated` when evaluation completes. |
| L4-G-010-BH-005 | Require Executive Reviewer confirmation before `Reviewed → Closed`. |
| L4-G-010-BH-006 | Publish `OperationalMaturityAssessmentClosed` on closure. |

## 5. Cross-Domain & Event Behavior
G00 publishes maturity events; source domains provide evidence only.

## 6. Failure / Rejection / Continuation
Incomplete required evidence blocks evaluation/closure; no classification outside the active framework may be invented.

## 7. Audit & Evidence Behavior
Preserve framework version, evidence references, derived result, reviewer decision and lifecycle history.

## 8. Observable Completion Conditions
Assessment is Closed with a reproducible maturity result.

## 9. Traceability & Provenance
Parent: L3-G-010; governing L2-G00 amendment dated 2026-09-01.

## 10. Open Issues / Deferred Design
Institution-specific maturity framework content remains configuration, not L4 design.
