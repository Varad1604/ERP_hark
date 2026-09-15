# L5-B-01-001 — Enterprise Search & Indexing Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-B-01-001 |
| Specification Title | Enterprise Search & Indexing Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-B — Data & Information Architecture |
| Major Area | L5-B-01 — Search & Indexing Architecture |
| Canonical Technical Responsibility | Enterprise technical search/indexing over authorized canonical sources; search never owns indexed facts. |
| Technical Architecture Owner | Enterprise Search & Indexing Technical Architecture |
| Assigned Author | Varad |
| Version | 1.0.1 |
| Status | Draft |
| Last Updated | 2026-09-15 |
| Depends On | L5-B-02-001; L5-C-02-001; L5-C-02-002; L5-D-02-001; L5-D-02-004; L5-D-03-001; L4-H-017; L4-H-018; L5 governance/template/validation standards |
| Referenced By | L5-B-01-002 and authorized enterprise search consumers |
| Applicable ADRs | ADR-L5-0005; ADR-L5-0007; ADR-L5-0009; ADR-L5-0010; ADR-L5-0014 |

## 2. Purpose & Boundary

Defines read-only enterprise search over disposable projections derived from canonical owners. Search accelerates discovery but never becomes an alternative source of truth, never mutates canonical records and never bypasses authorization or canonical owner contracts when an index is unavailable.

## 3. Canonical Constraints

- Indexed facts remain owned by their source domains/persistence owners.
- Search results are projections and may be stale; callers requiring authoritative state must resolve the returned canonical reference through the owning-domain contract.
- Authorization-aware filtering consumes D02 policy/entitlement decisions; search does not invent access policy.
- Search outage or index incompleteness must not trigger direct relational-table reads by B01. Fallback is either an owner-provided governed lookup capability or an explicit degraded/unavailable outcome.
- Latency, page-size, shard/replica topology and product selection are not canonical without approved authority.

## 4. Technical Realization

| Element | Responsibility |
|---|---|
| `SearchRequestAdmission` | Validate tenant, caller context, search scope and requested entity families. |
| `AuthorizationFilterBinder` | Consume D02-derived authorization constraints applicable to the search request. |
| `SearchProjectionQueryBoundary` | Query the B01-owned search projection only; no direct canonical-table access. |
| `ResultReferenceAssembler` | Return matching projection data plus canonical owner/reference and freshness metadata. |
| `AuthoritativeResolutionHint` | Identifies the owning-domain lookup path/reference when the caller requires authoritative state. |
| `SearchDegradationBoundary` | Returns degraded/unavailable or invokes an explicitly owner-provided lookup contract where such a contract exists; never scans persistence directly. |

## 5. Ownership

Owns search projection/query semantics, ranking/search technical behavior and freshness metadata. Does not own canonical facts, authorization policy, owner-domain lookup semantics, persistence tables, event delivery or recovery.

## 6. Failure & Recovery

Index unavailable/corrupt: return degraded/unavailable or use a separately governed owner lookup contract. Authorization-context failure: return no result rather than broaden access. Stale projection: expose freshness and permit caller to resolve canonical state. Event-delivery recovery is delegated to C02/B01-002; no local broker retry policy is defined here.

## 7. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-B01-001 | Search indexes are disposable read-only projections and never canonical fact owners. | Canonical responsibility |
| AD-B01-002 | Search degradation may not bypass the canonical owner through direct relational access. | Repository ownership boundary |
| AD-B01-003 | Authorization filtering consumes D02 decisions and fails closed. | Security boundary |
| OD-B01-001 | Search product, ranking implementation, topology, page limits and latency targets remain unresolved pending governed downstream design. | Open decision |

## 8. Conformance Requirements

1. B01 shall not write canonical domain records.
2. B01 shall not directly query canonical relational tables as an outage fallback.
3. Every result shall retain a canonical owner/reference sufficient for authoritative resolution.
4. Authorization filtering shall consume D02 authority and fail closed.
5. Product/topology/numeric SLO choices shall not become normative without approved decision authority.