# Source Data Collection to Validated Regulatory Snapshot

## 1. Specification Metadata
**Specification ID:** L4-H-057  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** NEP, NAAC & AISHE Reporting  
**Realizes Scenario:** L3-H-057  
**Primary Business Owner:** NEP, NAAC & AISHE Reporting  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-H-057; L2-H08 — NEP, NAAC & AISHE Reporting Workflows; applicable Locked source-domain architecture; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall collect authoritative source-domain facts for H08 reporting, validate and reconcile them without changing source ownership, and create an immutable validated Reporting Snapshot or return the reporting work to governed correction/data collection.

## 3. Required Business Context
The Reporting Cycle is ready for H08 `DataCollection`; a published Reporting Framework and Report Template govern the cycle; applicable source reporting contracts exist; operational facts remain owned by the actual `VAR:Evidence-Owning Domain` resolved for the cycle.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-057-BH-001 | The system shall begin H08 `DataCollection` only for the governed cycle and shall obtain required reporting facts by reference from their authoritative evidence-owning domains. | L3-H-057 §§7–9 |
| L4-H-057-BH-002 | The system shall verify collection completeness and record missing reporting data without fabricating or modifying source-domain facts. | L3-H-057 §§9–12 |
| L4-H-057-BH-003 | When collection is complete, the system shall progress the Reporting Cycle `DataCollection → Validation` and execute structural, business, multi-domain and compliance validation required by H08. | L3-H-057 §§9,12–13 |
| L4-H-057-BH-004 | The system shall govern the Validation Result through `Created → Executing → Completed → Passed` or `Failed` and shall publish the corresponding H08 validation events. | L3-H-057 §§9,13–14 |
| L4-H-057-BH-005 | On validation failure, the system shall publish `ValidationFailed`, return the Reporting Cycle to `DataCollection`, preserve the failed validation evidence, and require source-owned correction/recollection before another validation attempt. | L3-H-057 §§10–14 |
| L4-H-057-BH-006 | On successful validation, the system shall publish `ValidationPassed`, generate the Reporting Snapshot, progress the snapshot `Building → Validated → Locked`, and progress the Reporting Cycle to `SnapshotCreated`. | L3-H-057 §§9,12–14 |
| L4-H-057-BH-007 | Once the Reporting Snapshot reaches `Locked`, the system shall prevent in-place modification; later correction shall create a new governed Reporting Snapshot version rather than overwrite the locked snapshot. | L3-H-057 §§11–13 |
| L4-H-057-BH-008 | The system shall not treat the phrase "active reporting cycle" as a new Reporting Cycle state; this scenario is anchored to the defined `DataCollection` lifecycle position. | L3-H-057 §§7,13,17 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-057-BH-009 | H08 shall publish/record the parent-supported events `DataCollectionStarted`, `ReportingDataCollected`, `DataCollectionCompleted`, `ReportingDataMissing`, `ValidationStarted`, `ValidationCompleted`, `ValidationPassed`, `ValidationFailed`, `ComplianceGapDetected`, `ReportingSnapshotGenerated`, and `ReportingSnapshotLocked` for the corresponding H08-owned facts. | L3-H-057 §14 |
| L4-H-057-BH-010 | Source-domain facts/events shall retain their actual canonical owners/publishers; H08 shall consume authoritative information through approved reporting contracts without becoming the operational system of record. | L3-H-057 §§5,12,14 |

## 6. Failure / Rejection / Continuation
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-057-BH-011 | Missing, inconsistent or invalid mandatory reporting information shall prevent successful snapshot completion and remain visible in the governed validation/correction history. | L3-H-057 §§10–11 |
| L4-H-057-BH-012 | If authoritative source state cannot be corrected or recollected, the system shall not represent the Reporting Snapshot as validated or locked. | L3-H-057 §§11–12,16 |

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-057-BH-013 | The system shall preserve source references, collection-completeness results, validation findings, correction/recollection lineage, Validation Result history, Reporting Snapshot version lineage, snapshot-lock evidence and evidence mappings needed to reconstruct the reporting outcome. | L3-H-057 §15 |

## 8. Observable Completion Conditions
The behavior is complete when either a validated and `Locked` H08 Reporting Snapshot has been created from authoritative source facts and the Reporting Cycle has reached `SnapshotCreated`, or failed validation has returned the work to governed `DataCollection` without changing source ownership.

## 9. Traceability & Provenance
**Parent:** L3-H-057 — Source Data Collection to Validated Regulatory Snapshot  
**Primary verification source:** L2-H08 — NEP, NAAC & AISHE Reporting Workflows.

## 10. Open Issues / Deferred Design
The parent records a non-blocking H08 business-actor definition gap and the historical phrase "active reporting cycle" although the canonical Reporting Cycle lifecycle contains no `Active` state. This L4 specification does not invent either a named approval actor or an `Active` cycle state. Reporting data model, transport and UI design remain deferred.
