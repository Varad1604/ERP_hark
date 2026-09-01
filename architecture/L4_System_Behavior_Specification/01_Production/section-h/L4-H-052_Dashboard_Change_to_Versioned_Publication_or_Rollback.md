# Dashboard Change to Versioned Publication or Rollback

## 1. Specification Metadata
**Specification ID:** L4-H-052  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Dashboard  
**Realizes Scenario:** L3-H-052  
**Primary Business Owner:** Enterprise Dashboard  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-H-052; L2-H07 — Enterprise Dashboard Workflows; `L2-H07_Dashboard_Event_Vocabulary_Normalization_Amendment_2026-09-01.md`; `DEPENDENCY_MAP.md`; applicable Locked source-domain architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern approved Dashboard composition changes through immutable H07 Dashboard Version publication and governed rollback while preserving source-domain facts, A04 authorization ownership and complete version lineage.

## 3. Required Business Context
A governed Dashboard exists; the composition change or rollback request is authorized; the candidate or historical rollback target is eligible; source projections remain authoritative at their canonical owners.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-052-BH-001 | The system shall create and validate a candidate Dashboard Version for an approved composition change without altering an existing published or locked version. | L3-H-052 §§7–9,12–13 |
| L4-H-052-BH-002 | Dashboard Version progression shall follow `Candidate → Validated → Approved → Published → Locked → Superseded → Archived`; no historical version shall be mutated or directly reactivated. | L3-H-052 §13; L2-H07 normalization amendment §2 |
| L4-H-052-BH-003 | The system shall make a new version current only after governed approval and publication, preserving prior versions and lineage. | L3-H-052 §§9,12–16 |
| L4-H-052-BH-004 | For governed rollback, the system shall select an eligible prior immutable version as the restored governed basis, preserve current and historical versions, and record the rollback without rewriting lifecycle history. | L3-H-052 §§10–13,15–16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-052-BH-005 | H07 shall publish `DashboardVersionPublished` when a Dashboard Version is published; `DashboardVersionLocked` shall record subsequent immutability and shall not substitute for the publication event. | L2-H07 normalization amendment §2 |
| L4-H-052-BH-006 | `DashboardPublished` remains the Dashboard aggregate publication fact and shall not replace `DashboardVersionPublished` for a Dashboard Version. | L2-H07 normalization amendment §2 |
| L4-H-052-BH-007 | H07 shall publish `DashboardVersionRolledBack` when the governed rollback fact is established. | L3-H-052 §14; L2-H07 normalization amendment §2 |
| L4-H-052-BH-008 | Source metrics/projections shall remain owned by their canonical domains and shall not be modified by Dashboard versioning or rollback. | L3-H-052 §§5,14 |

## 6. Failure / Rejection / Continuation
An invalid candidate, missing authority or unsupported rollback target shall not replace the last valid governed publication. Historical versions remain immutable and traceable.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-052-BH-009 | The system shall preserve candidate/prior version references, validation and approval evidence, publication/lock facts, rollback reason and authority, selected historical basis and complete version lineage. | L3-H-052 §15 |

## 8. Observable Completion Conditions
The behavior completes when a new approved version is published and governed as current, or a governed rollback is completed to an eligible prior immutable basis, with the exact H07 version events and full lineage preserved.

## 9. Traceability & Provenance
**Parent:** L3-H-052 — Dashboard Change to Versioned Publication or Rollback  
**Primary verification sources:** L2-H07 — Enterprise Dashboard Workflows; L2-H07 Dashboard Event Vocabulary Normalization Amendment — 2026-09-01.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation. Storage, rendering, rollback implementation and UI mechanics remain deferred.
