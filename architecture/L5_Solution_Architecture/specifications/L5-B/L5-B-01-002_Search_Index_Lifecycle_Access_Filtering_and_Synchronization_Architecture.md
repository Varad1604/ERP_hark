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
| Version | 1.1.0 |
| Status | Review |
| Last Updated | 2026-09-15 |
| Depends On | L5-B-01-001; L5-B-02-001; L5-C-02-001; L5-C-02-002; L5-D-02-001; L5-D-02-004; L5-D-03-001; L4-H-018; L4-H-019; L5 governance/template/validation standards |
| Referenced By | L5-B-01-001 and governed search projection consumers |
| Applicable ADRs | ADR-L5-0005; ADR-L5-0007; ADR-L5-0008; ADR-L5-0009; ADR-L5-0010; ADR-L5-0014 |

## 2. Purpose & Boundary

Defines how B01 search projections are created, refreshed, rebuilt, validated, switched and filtered while preserving three canonical boundaries: source owners remain authoritative for facts, C02 owns event delivery/recovery semantics, and D02 owns authorization policy.

## 3. Architectural Invariants

1. B01 synchronizes only from C02-delivered events or an explicit source-owner rebuild/export/read contract.
2. B01 does not scan authoritative domain tables, assume table columns, hold database snapshots or own source persistence semantics.
3. C02 owns delivery, ordering, redelivery, subscription and dead-letter/recovery mechanics; B01 owns only projection application.
4. Projection update/rebuild is idempotent at the B01 application boundary where source contracts provide stable change identity/reference.
5. Replacement projections are validated before activation; invalid replacement state cannot displace the current valid projection.
6. Authorization filtering consumes D02 authority and fails closed.
7. Projection retirement occurs only after activation/rollback safety requirements are satisfied; exact retention intervals are downstream decisions.

## 4. Synchronization Sources

B01 may synchronize from:

1. Governed events delivered by C02 under ADR-L5-0010; or
2. A source-owner-provided rebuild/export/read contract explicitly intended for projection reconstruction.

The reconstruction contract must be authoritative for what constitutes the projection source set. B01 must not infer source completeness from database structure or implementation conventions.

## 5. Technical Realization

| Element | Responsibility | Input | Output | Failure Boundary |
|---|---|---|---|---|
| `ProjectionEventApplier` | Apply successfully delivered source-owner changes idempotently to the correct B01 projection. | C02-delivered change | Updated projection state | Cannot redefine transport retry/order semantics. |
| `ProjectionCheckpointReference` | Record B01's last successfully applied logical synchronization position/reference. | Applied change/rebuild progress | B01 synchronization reference | Must not become a claim of broker-offset ownership. |
| `RebuildSourceAdapter` | Consume owner-provided rebuild/export stream or governed read contract. | Owner reconstruction source | Reconstruction records | No direct table scan fallback. |
| `ReplacementIndexBuilder` | Construct a replacement projection independently from the active projection. | Reconstruction source | Replacement projection | Build failure cannot corrupt active projection. |
| `ProjectionValidationBoundary` | Validate completeness/integrity using owner-provided reconstruction contract and B01 technical invariants. | Replacement projection | Validated/rejected replacement | Failed validation prevents activation. |
| `ProjectionActivationBoundary` | Atomically switch B01's logical active-projection reference after validation. | Validated replacement | New active projection | Partial activation must not expose ambiguous active state. |
| `AuthorizationFilterBinder` | Apply D02-derived access constraints to indexed/searchable projection fields. | Projection definition + D02 constraints | Authorized projection/filter rules | Fails closed on unresolved required policy. |
| `ProjectionRetirementBoundary` | Retire prior projection after governed activation and rollback-safety conditions are satisfied. | Prior projection | Retired projection state | Exact timing remains downstream governed. |

## 6. Event Application Semantics

- B01 application must tolerate duplicate delivery according to source change identity/reference without creating duplicate search facts.
- Out-of-order or incompatible changes are handled according to the source contract and C02 ordering guarantees; B01 must not invent broker semantics.
- A change that cannot be safely applied is contained and surfaced as a B01 projection failure; transport recovery remains C02-owned.
- Projection checkpoint/reference means “last successfully applied logical B01 synchronization point,” not ownership of a broker offset or consumer group.

## 7. Rebuild & Activation Semantics

```text
Owner reconstruction source
  -> RebuildSourceAdapter
  -> ReplacementIndexBuilder
  -> ProjectionValidationBoundary
  -> ProjectionActivationBoundary
  -> Search queries use new active projection
  -> Prior projection retained only as needed for safe rollback
  -> ProjectionRetirementBoundary
```

A failed rebuild or failed validation leaves the current valid active projection unchanged. Activation must prevent callers from observing an indeterminate mixture of old/new active projection identities.

## 8. Projection Validation

Validation may include, when supported by the owner contract:

- source-set completeness evidence;
- expected entity/reference coverage;
- projection schema/profile compatibility;
- authorization-filter applicability;
- canonical owner/reference integrity;
- duplicate/idempotency consistency;
- internal projection invariants.

Validation criteria must not invent business facts or bypass the source owner.

## 9. Authorization Filtering

- D02 remains the authority for policy and entitlement meaning.
- B01 may materialize/index authorization-relevant projection attributes only as permitted by D02/privacy constraints.
- If required authorization context cannot be established, affected search access fails closed.
- Rebuild/activation must not accidentally activate a projection lacking required authorization-filter compatibility.

## 10. Failure & Recovery

| Failure | Treatment |
|---|---|
| Delivered change incompatible with projection | Contain B01 application failure; C02 retains transport recovery. |
| Rebuild source unavailable | Keep current valid projection; rebuild remains incomplete. |
| Replacement validation fails | Do not activate replacement. |
| Authorization context/policy unavailable | Fail closed. |
| Active projection corruption | Rebuild from governed owner source; no direct table repair. |
| Activation failure | Preserve/recover a single unambiguous active projection reference. |
| Retirement failure | Keep prior projection non-active until safe cleanup can be performed; no canonical data impact. |

## 11. Ownership

### Owns

Projection schema internal to B01, idempotent application of delivered changes, reconstruction orchestration over owner-provided sources, active-projection switching, B01 synchronization references, freshness metadata and authorization-aware projection filtering.

### Does Not Own

Canonical domain facts/tables, database snapshot semantics, message broker offsets, subscriptions, retry/backoff, dead-letter infrastructure, C02 recovery policy, D02 policy definitions or source-owner reconstruction semantics.

## 12. Observability & Evidence

Technical evidence should support correlation of projection identity/version, source-owner reconstruction reference, last successful B01 synchronization reference, activation/retirement outcome, authorization-filter compatibility, rebuild validation outcome and failure classification. Numeric lag, rebuild-duration, batch-size and decommission targets remain downstream governed.

## 13. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-B01-010 | Event transport/recovery is C02-owned; B01 only applies successfully delivered changes. | ADR-L5-0010 / ownership boundary |
| AD-B01-011 | Full rebuild uses an owner-provided reconstruction contract, never direct table scanning assumptions. | Repository ownership boundary |
| AD-B01-012 | Replacement projections are validated before logical activation. | Defensible technical derivation |
| AD-B01-013 | B01 synchronization references do not transfer broker-offset ownership from C02. | Ownership-preserving technical derivation |
| OD-B01-010 | Search-index product, physical alias mechanism, rebuild chunk size, decommission interval and topology remain downstream governed choices. | Open decision |

## 14. Conformance Requirements

1. B01-002 shall not define broker retry, exponential backoff, DLQ or consumer-group mechanics owned by C02.
2. B01-002 shall not assume direct table scans, table columns or database pagination contracts.
3. Rebuild shall consume a governed source-owner export/read/reconstruction capability.
4. Authorization filtering shall consume D02 authority and fail closed.
5. Replacement projection must pass governed validation before activation.
6. A failed rebuild/activation must not corrupt the current valid projection.
7. Projection checkpoint/reference must not be represented as broker-offset ownership.
8. No fixed rebuild chunk size, decommission duration or product-specific alias mechanism is canonical without approved authority.

## 15. Architecture Acceptance Evidence

A conforming design must demonstrate duplicate-safe projection application, rebuild from an owner-governed source, validation-before-activation, fail-closed authorization filtering, preservation of the active valid projection on rebuild failure, and no ownership leakage into C02 broker recovery or source persistence.

## 16. Traceability Summary

B01-002 realizes synchronization and lifecycle for non-authoritative search projections while C02 remains event-delivery/recovery owner, D02 remains authorization-policy owner, B02/source domains remain persistence/fact owners, and B01-001 consumes only validated active projections.