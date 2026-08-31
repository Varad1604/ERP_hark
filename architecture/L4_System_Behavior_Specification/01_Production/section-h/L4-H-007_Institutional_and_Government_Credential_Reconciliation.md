# Institutional and Government Credential Reconciliation

## 1. Specification Metadata
**Specification ID:** L4-H-007  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** APAAR & DigiLocker  
**Realizes Scenario:** L3-H-007  
**Primary Business Owner:** APAAR & DigiLocker  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-007; L2-H01 — APAAR & DigiLocker Workflows; applicable Locked source-domain architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall reconcile authoritative institutional/H01 publication references with government records through H01 Synchronization while preserving both histories and source credential ownership.

## 3. Required Business Context
H01 Publication/government references; authoritative source credential facts; authorized reconciliation request; H01 Synchronization scope and supported discrepancy classifications.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-007-BH-001 | On a scheduled review, audit or recovery request, the system shall begin reconciliation only when relevant H01/government references and authoritative source facts are available, the request is authorized, and H01 Synchronization can start. | L3-H-007 §§7–9 |
| L4-H-007-BH-002 | The system shall compare authoritative institutional/H01 publication references with government records and classify the outcome as fully synchronized, missing publication, government mismatch, duplicate publication, revoked credential detected, or manual review required as applicable. | L3-H-007 §§9,12 |
| L4-H-007-BH-003 | Resolution shall occur only through an already authorized source or H01 path; unresolved discrepancies shall be routed to governed follow-up rather than corrected locally by H01. | L3-H-007 §§9–12 |
| L4-H-007-BH-004 | H01 Synchronization shall complete as `Completed` or `Failed`; source credentials and existing Publications shall not be silently changed by reconciliation. | L3-H-007 §§9,11–13 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-007-BH-005 | H01 shall publish `ExternalProviderSynchronizationStarted` when Synchronization starts, `ApaarSynchronizationCompleted` for the source-defined APAAR success case, or `SynchronizationFailed` for failure; no generic success event is invented for other government scopes. | L3-H-007 §14 |

## 6. Failure / Rejection / Continuation
If comparison cannot complete, the system shall preserve a `Failed` Synchronization. If source truth is disputed, the canonical source owner must resolve it before H01 can consume the corrected authoritative fact.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-007-BH-006 | The system shall preserve comparison scope, authoritative references, discrepancy classification, reconciliation report/status, any governed follow-up, Synchronization outcome and both histories. | L3-H-007 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-007-BH-007 | Completion shall require completed comparison, a supported discrepancy classification, authoritative resolution where an existing path defines it or governed follow-up otherwise, preserved histories, and a source-defined Synchronization outcome. | L3-H-007 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-007 — Institutional and Government Credential Reconciliation  
**Primary verification source:** L2-H01 — APAAR & DigiLocker Workflows.

## 10. Open Issues / Deferred Design
Only `ApaarSynchronizationCompleted` is defined as a named success event for the generic scope and no universal cross-domain discrepancy-resolution contract exists; L4 does not invent either.