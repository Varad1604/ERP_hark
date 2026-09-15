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
| Version | 1.0.0 |
| Status | Draft |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-B-02-001; L5-B-02-002; L5-C-02-001; L5-C-02-002; L5-D-02-001; L5-D-02-004; L5-D-03-001; L4-H-017; L4-H-018 |
| Referenced By | L5-B-01-002; global enterprise navigation; student and faculty discovery portals |
| Applicable ADRs | ADR-L5-0005; ADR-L5-0007; ADR-L5-0009; ADR-L5-0010 |

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the solution architecture for enterprise search and multi-entity discovery across SutraERP. The architecture provides high-performance, full-text, faceted, and relational technical search capabilities over authorized canonical sources. It strictly enforces that search indexes are read-only, disposable projections derived from authoritative domain databases; search never becomes the canonical owner of business records and cannot mutate domain entities.

### In Scope

- Enterprise technical query processing, tokenization, stemming, and relevance ranking.
- Context-aware query evaluation honoring caller identity, active Tenant, and Institution boundaries.
- Dynamic security filtering enforcing principal visibility and permission scopes at query time.
- Cross-entity discovery and associative relationship indexing across academic, financial, and workforce records.
- Index read-path availability, fail-safe degradation, and caching of public/directory search results.

### Out of Scope / Non-Goals

- Becoming a primary database or authoritative system of record for business facts (owned by domain services via L5-B-02).
- Direct mutation of domain database tables from search interactions.
- Defining institutional access policies, role assignments, or permission definitions (owned by L5-D-02).
- Physical search engine cluster deployment or infrastructure provisioning.
- UI/UX search bar visual design, autocomplete dropdown widgets, or page layouts (owned by L6).

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L4 | `L4-H-017` | Approved / Locked | `L4-H-017-BH-001` | Search query context and technical discovery processing | Core search query processing pipeline |
| L4 | `L4-H-017` | Approved / Locked | `L4-H-017-BH-002` | Index retrieval and source-reference-preserving result resolution | Projection lookup and entity reference binding |
| L4 | `L4-H-017` | Approved / Locked | `L4-H-017-BH-003` | Eligible-result execution boundary; filtering from authorization controls | Authorization-aware result filtering |
| L4 | `L4-H-017` | Approved / Locked | `L4-H-017-BH-004` | Technical discovery path does not mutate source entities | Read-only discovery boundary preservation |
| L4 | `L4-H-017` | Approved / Locked | `L4-H-017-BH-006` | Technical correlation and evidence for discovery execution | Search audit and query trace logging |
| L4 | `L4-H-017` | Approved / Locked | `L4-H-017-BH-007` | Completion of technical search execution with source-derived results | Search result envelope packaging |
| L4 | `L4-H-018` | Approved / Locked | `L4-H-018-BH-001` | Intake of source-domain change events for search projection update | Event-driven search projection consumption |
| L4 | `L4-H-018` | Approved / Locked | `L4-H-018-BH-002` | Search-side representation remains projection-only | Source ownership preservation |
| Decision | Corpus Register | Locked | `L5-B-01-001` | Enterprise search over authorized sources; search never owns facts | Canonical platform responsibility |
| Decision | Stage-4 Proceed | Locked | Stage-4 Gate | Mandates minimum dependencies: `B02 + D02 + C02` | Dependency-gate boundary enforcement |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | Search indexes are ephemeral projections; data loss in a search index shall never cause business data loss. | L5-B-02 Baseline; ADR-L5-0005 | Every indexed document must be 100% reconstructible from canonical domain databases. |
| C-002 | Search results must never reveal entities the calling principal is unauthorized to see. | L5-D-02 Baseline; ADR-L5-0009 | Search queries apply mandatory filter clauses derived from caller's D02 authorization context. |
| C-003 | Search execution shall never write or mutate business records. | L4-H-017-BH-004 | Search queries execute over strictly read-only index replicas. |

### 3.3 Assumptions

| ID | Assumption | Why Needed | Impact if False | Validation / Resolution |
|---|---|---|---|---|
| AS-001 | Domain entities emit durable change events via L5-C-02 upon state mutations. | Enables asynchronous search index synchronization. | Search projections become stale. | Consumes durable outbox events governed by ADR-L5-0007 and ADR-L5-0010. |
| AS-002 | Search engine product selection is technology-neutral at solution architecture level. | Allows relational full-text or dedicated search clusters. | Document defines architecture contracts rather than vendor APIs. | Governed by Open Decision OD-001. |

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 Owns

- Enterprise search query parser, lexical analyzer, and search execution pipeline.
- Search projection schemas, indexing mappings, and tokenization analyzers.
- Query-time security filter generation integrating with `L5-D-02` security scopes.
- Search result assembling, snippet highlighting, and relevance score computation.
- Search index availability management, read-replica routing, and circuit-breaking.

### 4.2 Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-B-02-001` | Database & Persistence Architecture Baseline | Projection boundary | Preserves domain persistence ownership; defines outbox handoff (ADR-L5-0005). |
| `L5-B-02-002` | Domain Persistence Ownership & Isolation | Source of truth | Ensures search never reads directly from domain internal tables bypassing APIs. |
| `L5-C-02-001` | Event & Messaging Architecture Baseline | Durable events | Consumes domain entity update events to keep projections current (ADR-L5-0010). |
| `L5-C-02-002` | Durable Event Delivery & Ordering | Subscription ordering | Guarantees ordered consumption of entity mutations for index updates. |
| `L5-D-02-001` | Authorization Architecture Baseline | Security context | Consumes principal permission sets and institutional scope boundaries (ADR-L5-0009). |
| `L5-D-03-001` | Canonical Trust Boundary & Segmentation | Network boundary | Enforces segmentation between public search endpoints and index storage clusters. |

### 4.3 Must Not Own

- Authoritative entity state, master data, or historical transaction records.
- Identity authentication or user session management (owned by L5-D-01).
- Authorization policy evaluation or permission granting (owned by L5-D-02).
- Business workflow routing or domain command processing.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| `SearchQueryCoordinator` | Control Point | Admits query request, validates tenant/institution scope, and enforces query quotas. | `SearchQueryRequest` | Stateless | Admitted query context | `L5-A-04`, `L5-D-01` | Rejects malformed queries or missing tenant context. | L4-H-017-BH-001 |
| `SecurityFilterAugmenter` | Component | Translates caller's D02 authorization tokens into mandatory index filter predicates. | Caller security tokens | Stateless | Security-augmented query | `L5-D-02-004` | Restricts search space to authorized records only. | C-002, ADR-L5-0009 |
| `IndexQueryExecutionEngine` | Runtime Component | Executes full-text, boolean, and faceted queries against read-only search index replicas. | Filtered query AST | Read-only index | Raw search hits & scores | Search backend | Fails over to secondary replica on node timeout. | L4-H-017-BH-002 |
| `ResultAssembler` | Component | Resolves canonical entity references, generates highlighted snippets, and strips sensitive fields. | Raw hits & source metadata | Stateless | `SearchResultSet` | `L5-B-05-002` | Masks sensitive data before returning to client. | L4-H-017-BH-007 |
| `SearchCircuitBreaker` | Control Point | Monitors search cluster latency and error rates; engages graceful degradation when unavailable. | Engine telemetry | In-memory metrics | Degraded response / Error | `L5-E-01-002` | Returns empty/cached result or fallback directory search. | AD-B01-004 |

### 5.2 Primary Technical Flow

```text
User / Service Search Request (L4-H-017)
         ↓
SearchQueryCoordinator (validate Tenant, Rate Limits, Sanitize Query String)
         ↓
SecurityFilterAugmenter (inject D02 Scope: InstitutionId, DepartmentId, ConfidentialityLevel)
         ↓
IndexQueryExecutionEngine (execute against read-only Search Index Replica)
  [Index Failure] → SearchCircuitBreaker (trigger fallback or return ERR_SEARCH_UNAVAILABLE)
         ↓
ResultAssembler (filter unauthorized fields via L5-B-05, generate highlighted snippets)
         ↓
SearchAuditLogger (log query string, execution time, result count via L4-H-017-BH-006)
         ↓
Return SearchResultSet Envelope (Preserving canonical entity UUIDs)
```

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Authorization Token Missing | `SecurityFilterAugmenter` | Reject query (`ERR_SEARCH_UNAUTHORIZED`). | Executing unconstrained query revealing private records. | Client must supply valid D02 session token. | C-002, ADR-L5-0009 |
| Search Cluster Outage | `SearchCircuitBreaker` | Return structured service degradation (`ERR_SEARCH_SERVICE_DEGRADED`). | Crashing caller application or hanging request threads. | Route to direct relational indexed lookup for primary IDs. | AD-B01-004 |
| Index Projection Stale | `IndexQueryExecutionEngine` | Return available result with freshness timestamp flag. | Halting search completely because index is 2 seconds behind. | Background sync catches up via L5-B-01-002. | L4-H-018-BH-003 |
| Malformed Query Syntax | `SearchQueryCoordinator` | Sanitize or return validation error (`ERR_INVALID_QUERY_SYNTAX`). | Propagating raw syntax errors or injection strings to engine. | Query parser normalizes boolean operators. | L4-H-017-BH-001 |

### 5.4 Conceptual Contracts

#### `SearchQueryRequest`
```text
SearchQueryRequest {
  TenantId: UUID
  InstitutionId: UUID
  QueryCorrelationId: UUID
  CallingPrincipalId: UUID
  CallerSecurityScopes: List<String>
  QueryString: String
  TargetEntityTypes: List<String> (e.g., ["STUDENT", "COURSE", "DOCUMENT"])
  FacetsRequested: List<String>
  Pagination: {
    Offset: Integer
    Limit: Integer (Max: 100)
  }
}
```

#### `SearchResultSet`
```text
SearchResultSet {
  QueryCorrelationId: UUID
  TotalHitCount: Long
  ReturnedHitCount: Integer
  ExecutionDurationMs: Integer
  IndexFreshnessTimestamp: Timestamp
  Hits: List<SearchResultHit>
  Facets: Map<String, List<FacetBucket>>
  SearchStatus: Enum [SUCCESS, DEGRADED_FALLBACK, PARTIAL_TIMEOUT]
}

SearchResultHit {
  EntityType: String
  EntityId: UUID (Canonical domain reference)
  RelevanceScore: Float
  Title: String
  SummarySnippet: String
  HighlightedMatches: Map<String, List<String>>
  DirectAccessUri: String
}
```

### 5.5 State, Persistence & Consistency Boundaries

Search indexes are **derived, disposable read projections**. They possess eventual consistency with respect to domain databases. Updates are propagated asynchronously via durable domain events (`L5-C-02`). In the event of catastrophic index corruption, indexes are discarded and completely reconstructed from authoritative persistence (`L5-B-02`).

### 5.6 Asynchronous & Job Realization

Search query evaluation is purely synchronous on the read path. Re-indexing, index optimization, and bulk re-projections operate asynchronously in background workers governed by `L5-A-08-001`.

### 5.7 Security, Trust & Isolation

- Search results are strictly scoped by Tenant and Institution ID at the index partition level.
- Multi-tenancy is enforced either through tenant-isolated index collections or mandatory routing filters.
- Sensitive fields (e.g., Aadhaar number, employee bank account, confidential medical notes) are strictly excluded from index schemas (`L5-B-05-002`).

### 5.8 Runtime & Deployment Implications

Search clusters deploy in dedicated read-replica topologies (`L5-E-04-001`), completely isolated from transaction-processing database servers. Read traffic scales horizontally by adding search replica nodes.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-B01-001` | Derived Read-Only Projection | Search indexes are treated strictly as ephemeral projections; zero domain write authority. | L5-B-02 Baseline; C-001 | Guarantees that search index corruption or deletion never compromises primary enterprise data integrity. | Governed by ADR-L5-0005 |
| `AD-B01-002` | Mandatory Query-Time Security Filtering | Caller authorization context is translated into mandatory boolean filter clauses appended to every query. | L4-H-017-BH-003; C-002 | Guarantees that principals can never discover or view records outside their authorized scope. | Governed by ADR-L5-0009 |
| `AD-B01-003` | Canonical UUID Result Reference | Search results return canonical entity UUIDs and resource URIs rather than full enterprise entity models. | L4-H-017-BH-002 | Forces callers to fetch authoritative domain entities through domain APIs when performing state mutations. | None (Local L5 Architecture Decision) |
| `AD-B01-004` | Graceful Degradation on Search Outage | When search clusters are offline, fallback routing resolves exact-match queries via relational primary key lookups. | L5-E-01-002; L4-H-017 | Preserves basic operational navigation during full-text search infrastructure maintenance. | Governed by ADR-L5-0014 |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Security Scope Enforcement | Query evaluation guarantees zero leakage of cross-tenant or unauthorized records. | Enterprise security standard | `L5-D-02-004`, `L5-A-04-001` |
| Sensitive Data Masking | Sensitive PII and salary fields are omitted from index mappings; snippets are sanitized. | Privacy compliance | `L5-B-05-002` |
| Query Availability & Latency | Read replicas decouple search load from relational transactional processing; p95 latency < 50ms. | System performance target | `L5-E-01-001`, `L5-E-02-001` |

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| `CR-001` | Search query processing shall strictly enforce TenantId and InstitutionId isolation on every index query. | §5.1, §5.7 | `L4-H-017-BH-001` |
| `CR-002` | The engine shall inject mandatory security scope filters derived from caller D02 authorizations before query execution. | §5.1, `AD-B01-002` | `L4-H-017-BH-003`, C-002 |
| `CR-003` | Search execution shall be strictly read-only and shall not execute database mutations or invoke domain commands. | §4.3, `AD-B01-001` | `L4-H-017-BH-004`, C-003 |
| `CR-004` | All search hits shall return canonical entity UUIDs referencing the authoritative domain aggregate. | §5.1, `AD-B01-003` | `L4-H-017-BH-002` |
| `CR-005` | Search index schemas shall explicitly exclude sensitive PII, passwords, and Aadhaar numbers. | §5.7 | `L5-B-05-002` |
| `CR-006` | In the event of complete search cluster unavailability, the system shall degrade gracefully without failing application startup. | §5.3, `AD-B01-004` | ADR-L5-0014 |
| `CR-007` | Search query strings shall be sanitized to prevent query-injection or syntax crash attacks. | §5.1, §5.3 | `L4-H-017-BH-001` |
| `CR-008` | Search result snippets and highlights shall be sanitized to prevent Cross-Site Scripting (XSS). | §5.1 | `L5-D-03-001` |
| `CR-009` | The search engine shall emit technical observability logs covering query latency and hit counts. | §5.1, §5.4 | `L4-H-017-BH-006` |
| `CR-010` | The implementation shall support paginated query execution with a hard limit of 100 hits per page. | §5.4 | `L5-E-02-001` |
| `CR-011` | Search index mappings shall be reconstructible from canonical persistence tables using defined rebuild jobs. | §5.5, `AD-B01-001` | C-001 |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| `OD-001` | Search Engine Technology Selection | ADR-Required | PostgreSQL pg_trgm/FTS vs OpenSearch vs Meilisearch | Lead Architect / L5-B-01 | No | Evaluate operational complexity and resource footprint in Stage 6 consolidation. |
| `OD-002` | Semantic Vector Search / Embedding Integration | Extensibility | Vector search for semantic institutional policy discovery | Architecture Board | No | Vector embeddings deferred to future analytics and AI assistant stage. |
| `OD-003` | Typo-Tolerance and Fuzzy Search Levenshtein Distance | Specification | Standard edit distance threshold across entity types | L5-B-01 Owner | No | Standardize fuzzy distance (1 vs 2) during L6 API design. |

---

## 10. Traceability Summary

- `L4-H-017-BH-001` — Realized by `SearchQueryCoordinator` admitting and sanitizing discovery queries (§5.1, `CR-001`, `CR-007`).
- `L4-H-017-BH-002` — Realized by `IndexQueryExecutionEngine` and `ResultAssembler` resolving canonical entity references (§5.1, `CR-004`).
- `L4-H-017-BH-003` — Realized by `SecurityFilterAugmenter` injecting authorization scope filters (§5.1, `CR-002`).
- `L4-H-017-BH-004` — Realized by read-only query execution boundary preventing domain entity mutation (§5.1, `CR-003`).
- `L4-H-017-BH-006` — Realized by execution telemetry and query trace logging (§5.1, `CR-009`).
- `L4-H-017-BH-007` — Realized by `SearchResultSet` packaging and delivery to calling clients (§5.4).
- `L4-H-018-BH-001` — Realized by event-driven search projection consumption boundary (§4.2).
- `L4-H-018-BH-002` — Realized by projection-only state model preserving domain persistence authority (§5.5, `CR-011`).
- `L4-H-018-BH-003` — Realized by index freshness timestamp tracking and eventual consistency management (§5.4, §5.5).
