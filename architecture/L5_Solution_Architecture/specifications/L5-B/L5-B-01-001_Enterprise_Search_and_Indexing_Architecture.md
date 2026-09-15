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
| Version | 1.1.0 |
| Status | Review |
| Last Updated | 2026-09-15 |
| Depends On | L5-B-02-001; L5-C-02-001; L5-C-02-002; L5-D-02-001; L5-D-02-004; L5-D-03-001; L4-H-017; L4-H-018; L5 governance/template/validation standards |
| Referenced By | L5-B-01-002 and authorized enterprise search consumers |
| Applicable ADRs | ADR-L5-0005; ADR-L5-0007; ADR-L5-0009; ADR-L5-0010; ADR-L5-0014 |

## 2. Purpose & Boundary

Defines read-only enterprise search over disposable projections derived from canonical owners. Search accelerates discovery but never becomes an alternative source of truth, never mutates canonical records and never bypasses authorization or canonical owner contracts when an index is unavailable.

## 3. Architectural Invariants

1. Search projections are non-authoritative and disposable; canonical facts remain with source owners.
2. Every search result retains enough canonical owner/reference information for authoritative resolution.
3. Authorization-aware filtering consumes D02 authority and fails closed when required authorization context cannot be established.
4. Index unavailability or staleness never authorizes direct relational-table access by B01.
5. Search freshness/degradation must be explicit enough that callers can distinguish discovery from authoritative state resolution.
6. Ranking/search relevance may order projected facts but cannot alter their canonical business meaning.
7. Tenant/institution isolation applies to projection creation, query evaluation and result return.

## 4. Search Projection Model

A search projection may contain denormalized, tokenized, derived or ranking-oriented data needed for discovery, provided that:

- the source canonical owner remains identifiable;
- the projected representation does not become the authoritative record;
- stale or incomplete projection state can be represented explicitly;
- sensitive fields are indexed only when authorized by applicable privacy/security policy;
- authorization filtering is applied before unauthorized data can be returned;
- rebuild or deletion of the projection does not affect canonical business records.

## 5. Technical Realization

| Element | Responsibility | Input | Output | Failure Boundary |
|---|---|---|---|---|
| `SearchRequestAdmission` | Validate tenant, caller context, search scope, correlation identity and requested entity families. | Search request | Admitted search request | Rejects missing/invalid scope or authorization context. |
| `AuthorizationFilterBinder` | Consume D02-derived authorization constraints applicable to the search request. | Caller/policy context | Bound search filters | Failure to resolve required policy fails closed. |
| `SearchProjectionQueryBoundary` | Query only the B01-owned search projection. | Admitted request + filters | Candidate result set | Never direct-queries canonical persistence as fallback. |
| `RankingAndMatchBoundary` | Apply technical matching/ranking over authorized projected data. | Candidate projected records | Ranked result references | Cannot redefine business meaning or authority. |
| `ResultReferenceAssembler` | Return matching projection data plus canonical owner/reference and freshness metadata. | Ranked results | Search result envelope | Result without authoritative reference is invalid for canonical resolution. |
| `AuthoritativeResolutionHint` | Identify the owning-domain lookup/reference required when authoritative state is needed. | Search result | Owner-resolution reference | Does not implement owner-domain semantics itself. |
| `SearchDegradationBoundary` | Return degraded/unavailable outcome or invoke an explicitly owner-provided governed lookup contract where such a contract exists. | Search failure/degradation | Safe degraded response | Never scans persistence directly. |

## 6. Conceptual Search Contract

A search request conceptually identifies tenant/institution, caller/authorization context, search scope/entity family, query criteria, correlation identity and requested projection fields/profile.

A search result conceptually returns projected match information, canonical owner/reference, freshness/projection version reference, authorization-filtered field set and degradation/freshness indication where applicable.

Exact query syntax, pagination fields, ranking expressions and API shapes remain downstream design.

## 7. Freshness & Consistency Semantics

- Search operates under projection consistency, not canonical transactional consistency.
- A result may be stale relative to its canonical owner; the caller must resolve authoritative state when business correctness depends on current facts.
- Freshness metadata must be sufficient to indicate the projection state/reference used without imposing an invented universal time threshold.
- A projection lag condition is a technical degradation condition, not a business-state change.
- Search must never claim that “not found” proves canonical absence unless the owning workflow explicitly authorizes that interpretation under known projection guarantees.

## 8. Authorization & Sensitive Data Handling

- Searchable/indexed fields must be restricted to data permitted by D02/privacy/classification authority.
- Authorization filtering must occur before unauthorized information is returned; post-return masking is not a substitute for access control.
- Authorization-context failure fails closed and must not broaden the search scope.
- Sensitive fields should be minimized in projections when discovery can be satisfied by less-sensitive references/metadata.
- Public/external search, if later required, must use separately governed exposure/abuse controls rather than inheriting internal search assumptions.

## 9. Degradation & Failure Model

| Failure | Required Treatment |
|---|---|
| Index unavailable/corrupt | Return degraded/unavailable or use separately governed owner lookup contract. |
| Authorization context unavailable | Fail closed; return no unauthorized data. |
| Projection stale/lagging | Expose freshness/degradation and allow authoritative resolution. |
| Projection incomplete | Do not claim canonical absence; surface degraded/incomplete condition as applicable. |
| Owner lookup unavailable | Return unresolved/degraded; do not query persistence directly. |
| Search processing failure | Contain to search boundary; canonical data remains unaffected. |

## 10. Ownership

### Owns

Search projection/query semantics, ranking/matching technical behavior, result reference assembly, freshness metadata and B01-owned degradation behavior.

### Does Not Own

Canonical facts, authorization policy, owner-domain lookup semantics, persistence tables, event delivery/recovery, source-domain lifecycle or business interpretation of search results.

## 11. Observability & Operability

Search technical evidence should support correlation of request scope, authorization-filter application, projection/version reference, freshness/degradation state, result-count/processing outcome and failure class without logging unnecessary sensitive search terms or result content. Exact latency/throughput/page-size targets remain downstream governed decisions.

## 12. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-B01-001 | Search indexes are disposable read-only projections and never canonical fact owners. | Canonical responsibility |
| AD-B01-002 | Search degradation may not bypass the canonical owner through direct relational access. | Repository ownership boundary |
| AD-B01-003 | Authorization filtering consumes D02 decisions and fails closed. | Security boundary |
| AD-B01-004 | Search absence/staleness cannot be treated as authoritative canonical absence unless explicitly guaranteed by the owning workflow. | Repository-consistent technical derivation |
| OD-B01-001 | Search product, ranking implementation, topology, page limits and latency targets remain unresolved pending governed downstream design. | Open decision |

## 13. Conformance Requirements

1. B01 shall not write canonical domain records.
2. B01 shall not directly query canonical relational tables as an outage fallback.
3. Every result shall retain a canonical owner/reference sufficient for authoritative resolution.
4. Authorization filtering shall consume D02 authority and fail closed.
5. Search staleness/incompleteness shall not be represented as authoritative business truth.
6. Sensitive indexed fields shall remain governed by authorization/privacy/classification controls.
7. Product/topology/numeric SLO choices shall not become normative without approved decision authority.

## 14. Architecture Acceptance Evidence

A conforming design must demonstrate that index deletion/rebuild cannot affect canonical records, authorization failure cannot broaden access, stale/incomplete search cannot be misrepresented as authoritative absence, and index outage cannot cause direct persistence-table fallback.

## 15. Traceability Summary

B01 provides discovery over governed projections while source owners retain canonical fact authority. D02 supplies access-policy authority, C02/B01-002 supply projection synchronization paths, B02 remains canonical persistence owner, and callers requiring current business truth resolve the canonical reference through the owning domain.