# Authoritative Source Change to Current Search Projection

## 1. Specification Metadata
**Specification ID:** L4-H-018  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Directory & Search  
**Realizes Scenario:** L3-H-018  
**Primary Business Owner:** Enterprise Directory & Search  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-018; L2-H03 — Enterprise Directory & Search Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall synchronize an authoritative source change into the H03 searchable projection while preserving source-domain ownership, business meaning and lifecycle.

## 3. Required Business Context
Authoritative source change; canonical source reference/owner; existing Searchable Projection; indexing/synchronization contract; security/synchronization metadata.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-018-BH-001 | On an authoritative searchable source fact or correction, H03 shall resolve the existing projection by canonical reference/source ownership and validate the change against the indexing contract and required metadata. | L3-H-018 §§7–9 |
| L4-H-018-BH-002 | H03 shall update only its searchable representation/metadata and shall not modify or reinterpret the source entity, rules or lifecycle. | L3-H-018 §§5,9,12 |
| L4-H-018-BH-003 | A valid canonical update shall progress the projection `Searchable → Updated → Searchable` after successful reindex/update completion, with current visibility rules applied before search availability. | L3-H-018 §§9–13 |
| L4-H-018-BH-004 | Contract-validation failure shall retain the prior valid projection; stale/inconsistent or source-uncertain cases shall enter governed recovery rather than guessed synchronization. | L3-H-018 §§10–11 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-018-BH-005 | H03 shall publish `SearchProjectionUpdated` and `SearchIndexUpdated` for its completed update facts where applicable; no universal upstream publisher is assigned to generic `CanonicalEntityUpdated` because actual source-domain events differ. | L3-H-018 §14 |

## 6. Failure / Rejection / Continuation
Invalid contract/security metadata or unclear source ownership prevents projection activation; source archive/removal follows its separate source-driven H03 lifecycle.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-018-BH-006 | The system shall preserve projection identity, canonical source reference/owner, source-version/synchronization lineage, validation result and resulting search visibility. | L3-H-018 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-018-BH-007 | Completion shall require a contract-valid source change, a current permitted H03 projection, valid security/visibility metadata and traceable synchronization lineage while source ownership remains unchanged. | L3-H-018 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-018 — Authoritative Source Change to Current Search Projection  
**Primary verification source:** L2-H03 — Enterprise Directory & Search Workflows.

## 10. Open Issues / Deferred Design
The parent retains generic upstream-event/publisher and exceptional administrative-actor gaps; no universal values are invented.