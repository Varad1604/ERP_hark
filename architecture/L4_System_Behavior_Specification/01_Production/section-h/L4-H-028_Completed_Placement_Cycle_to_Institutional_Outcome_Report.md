# Completed Placement Cycle to Institutional Outcome Report

## 1. Specification Metadata
**Specification ID:** L4-H-028  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Placement  
**Realizes Scenario:** L3-H-028  
**Primary Business Owner:** Placement  
**Version:** 1.1.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-H-028; L2-H04 Placement Outcome Reporting Clarification

## 2. Behavioral Objective & Boundary
Make H04-supported Placement statistics available from completed Placement outcomes without inventing diversity/report-publication architecture.

## 3. Required Business Context
A Placement cycle/drive scope is Closed and relevant Placement outcomes are in governed completed positions; C/E facts remain source-owned.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior |
|---|---|
| L4-H-028-BH-001 | Resolve the closed Placement scope and eligible completed Placement outcomes. |
| L4-H-028-BH-002 | Exclude/defer incomplete or unreconciled outcomes rather than infer them. |
| L4-H-028-BH-003 | Reference permitted C/E facts without copying canonical ownership. |
| L4-H-028-BH-004 | Generate only H04-supported Placement statistics/read-model facts. |
| L4-H-028-BH-005 | Publish `PlacementStatisticsGenerated` when the governed statistics projection is available. |
| L4-H-028-BH-006 | Make the projection available only to authorized institutional consumers. |

## 5. Cross-Domain & Event Behavior
H04 publishes Placement statistics; H07 may consume for dashboard composition but does not own the statistics.

## 6. Failure / Rejection / Continuation
Unsupported diversity dimensions or unreconciled metrics are not generated.

## 7. Audit & Evidence Behavior
Preserve reporting scope, source outcomes, excluded items, C/E references and authorization context.

## 8. Observable Completion Conditions
`PlacementStatisticsGenerated` exists for a closed governed scope and the supported projection is authorized for institutional use.

## 9. Traceability & Provenance
Parent: L3-H-028; H04 reporting clarification dated 2026-09-01.

## 10. Open Issues / Deferred Design
Additional diversity analytics require separate future architecture change control.
