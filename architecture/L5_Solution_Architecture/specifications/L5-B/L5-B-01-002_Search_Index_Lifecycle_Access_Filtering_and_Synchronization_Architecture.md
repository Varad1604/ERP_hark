# L5-B-01-002 — Search Index Lifecycle, Access Filtering & Synchronization Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-B-01-002 |
| Specification Title | Search Index Lifecycle, Access Filtering & Synchronization Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-B — Data & Information Architecture |
| Major Area | L5-B-01 — Search & Indexing Architecture |
| Canonical Technical Responsibility | Index lifecycle/freshness/synchronization and authorization-aware filtering without owning event delivery or canonical persistence. |
| Technical Architecture Owner | Search Synchronization & Lifecycle Technical Architecture |
| Assigned Author | Varad |
| Version | 1.0.1 |
| Status | Locked |
| Last Updated | 2026-09-15 |
| Depends On | L5-B-01-001; L5-B-02-001; L5-C-02-001; L5-C-02-002; L5-D-02-001; L5-D-02-004; L5-D-03-001; L4-H-018; L4-H-019; L5 governance/template/validation standards |
| Referenced By | L5-B-01-001 and governed search projection consumers |
| Applicable ADRs | ADR-L5-0005; ADR-L5-0007; ADR-L5-0008; ADR-L5-0009; ADR-L5-0010; ADR-L5-0014 |

## 2. Purpose & Boundary

Defines how B01 search projections are created, refreshed, rebuilt, switched and filtered while preserving three canonical boundaries: source owners remain authoritative for facts, C02 owns event delivery/recovery semantics, and D02 owns authorization policy.

## 3. Synchronization Sources

B01 may synchronize from:

1. Governed events delivered by C02 under ADR-L5-0010; or
2. A source-owner-provided rebuild/export/read contract explicitly intended for projection reconstruction.

B01 shall not scan authoritative domain tables directly, assume `updated_at`/primary-key pagination, hold database snapshots itself or invent owner persistence schemas.

C02 owns consumer delivery, ordering, redelivery, dead-letter/recovery and subscription mechanics. B01 owns only application-level projection application and the decision whether an event can be applied to the index.

## 4. Technical Realization

| Element | Responsibility |
|---|---|
| `ProjectionEventApplier` | Apply C02-delivered owner events idempotently to the correct B01 projection. |
| `ProjectionCheckpointReference` | Record B01's last successfully applied logical synchronization position/reference without redefining broker offset ownership. |
| `RebuildSourceAdapter` | Consume an owner-provided rebuild/export stream or governed read contract. |
| `ReplacementIndexBuilder` | Construct a replacement projection independently from the active projection. |
| `ProjectionValidationBoundary` | Validate completeness/integrity according to owner-provided reconstruction contract and B01 technical invariants. |
| `ProjectionActivationBoundary` | Atomically switch B01's logical active-projection reference after validation. |
| `AuthorizationFilterBinder` | Apply D02-derived access constraints to indexed/searchable projection fields. |
| `ProjectionRetirementBoundary` | Retire prior projection only after governed activation and when no longer required for safe rollback; exact retention duration is not defined here. |

## 5. Ownership

### Owns
Projection schema internal to B01, idempotent application of delivered changes, rebuild orchestration over owner-provided sources, active-projection switching, freshness metadata and authorization-aware projection filtering.

### Does Not Own
Canonical domain facts/tables, database snapshot semantics, message broker offsets, subscription delivery, retry/backoff, dead-letter infrastructure, C02 recovery policy, D02 policy definitions or fixed rebuild batch sizes.

## 6. Failure & Recovery

- Event cannot be applied because of projection incompatibility: stop/contain the affected projection update and surface a B01 technical failure; C02 remains owner of transport redelivery/recovery.
- Rebuild source unavailable: rebuild remains incomplete; active valid projection is not replaced.
- Replacement validation fails: do not activate it.
- Authorization policy/context unavailable: fail closed; do not expose unfiltered results.
- Active projection corruption: rebuild from governed owner source; do not repair by reading tables directly.

## 7. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-B01-010 | Event transport/recovery is C02-owned; B01 only applies successfully delivered events. | ADR-L5-0010 / ownership boundary |
| AD-B01-011 | Full rebuild uses an owner-provided reconstruction contract, never direct table scanning assumptions. | Repository ownership boundary |
| AD-B01-012 | Replacement projections are validated before logical activation. | Defensible technical derivation |
| OD-B01-010 | Search-index product, physical alias mechanism, rebuild chunk size, decommission interval and topology remain downstream governed choices. | Open decision |

## 8. Conformance Requirements

1. B01-002 shall not define broker retry, exponential backoff, DLQ or consumer-group mechanics owned by C02.
2. B01-002 shall not assume direct table scans, table columns or database pagination contracts.
3. Rebuild shall consume a governed source-owner export/read/reconstruction capability.
4. Authorization filtering shall consume D02 authority and fail closed.
5. No fixed rebuild chunk size, decommission duration or product-specific alias mechanism is canonical without approved authority.