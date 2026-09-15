# L5-B-01-002 — Search Index Lifecycle, Access Filtering & Synchronization Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-B-01-002 |
| Specification Title | Search Index Lifecycle, Access Filtering & Synchronization Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-B — Data & Information Architecture |
| Major Area | L5-B-01 — Search & Indexing Architecture |
| Canonical Technical Responsibility | Index lifecycle/freshness/synchronization and authorization-aware filtering. |
| Technical Architecture Owner | Search Synchronization & Lifecycle Technical Architecture |
| Assigned Author | Varad |
| Version | 1.0.0 |
| Status | Draft |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-B-01-001; L5-B-02-001; L5-B-02-002; L5-C-02-001; L5-C-02-002; L5-D-02-001; L5-D-02-004; L5-D-03-001; L4-H-018; L4-H-019 |
| Referenced By | L5-B-01-001; all domain event publishers; disaster recovery and re-indexing workflows |
| Applicable ADRs | ADR-L5-0005; ADR-L5-0007; ADR-L5-0008; ADR-L5-0010; ADR-L5-0014 |

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the solution architecture for synchronizing enterprise search indexes with canonical domain data and managing search index lifecycles across SutraERP. It establishes the durable event-driven ingestion pipeline, projection versioning, blue/green index rebuilding, access-filtering token generation, and disaster recovery rebuilds, ensuring that search projections remain synchronized, secure, and fully recoverable from canonical persistence truth.

### In Scope

- Event-driven index synchronization consuming canonical domain change events via `L5-C-02`.
- Transformation of domain entity state changes into optimized search projection documents.
- Projection schema versioning, blue/green index rebuilding, and atomic index alias swapping.
- Pre-computing and embedding access-filtering tokens (tenant, institution, department, roles) into projection records.
- Complete index recovery, reconciliation, and verified rebuilding from authoritative domain databases (`L5-B-02`).
- Tracking index synchronization lag, consumer offsets, and projection freshness metrics.

### Out of Scope / Non-Goals

- Executing user search queries or ranking algorithms (owned by L5-B-01-001).
- Owning domain business entities or publishing domain business events (owned by upstream domains).
- Defining access control policies or user role bindings (owned by L5-D-02).
- Managing primary database replication or relational backups (owned by L5-B-02 and L5-E-03).
- Direct mutation of canonical domain entities during index synchronization.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L4 | `L4-H-018` | Approved / Locked | `L4-H-018-BH-001` | Source-change intake, source/version resolution, and synchronization admission | Event intake and deduplication pipeline |
| L4 | `L4-H-018` | Approved / Locked | `L4-H-018-BH-002` | Projection-only synchronization and preservation of source ownership | Non-destructive projection updates |
| L4 | `L4-H-018` | Approved / Locked | `L4-H-018-BH-003` | Currentness, lifecycle-controlled update, and eligible-index gating | Index freshness and active index routing |
| L4 | `L4-H-018` | Approved / Locked | `L4-H-018-BH-004` | Contract/source uncertainty containment and governed recovery | Error quarantine and poison message handling |
| L4 | `L4-H-018` | Approved / Locked | `L4-H-018-BH-006` | Source/version/synchronization lineage and technical outcome evidence | Sync checkpoint and offset tracking |
| L4 | `L4-H-018` | Approved / Locked | `L4-H-018-BH-007` | Validated synchronization and eligible current representation | Projection validation before query exposure |
| L4 | `L4-H-019` | Approved / Locked | `L4-H-019-BH-001` | Preservation of affected projection identity and lineage before recovery | Pre-recovery snapshot and isolation |
| L4 | `L4-H-019` | Approved / Locked | `L4-H-019-BH-002` | Authoritative-source rebuild and reconciliation architecture | Batch re-indexing from canonical DB |
| L4 | `L4-H-019` | Approved / Locked | `L4-H-019-BH-003` | Rebuild/activation technical lifecycle and recovery gating | Staging index build and verification |
| L4 | `L4-H-019` | Approved / Locked | `L4-H-019-BH-004` | Validation gate before recovered content becomes searchable | Atomic alias swap and health verification |
| Decision | Corpus Register | Locked | `L5-B-01-002` | Index lifecycle, freshness, synchronization, and access filtering | Canonical platform responsibility |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | Ingestion of domain change events shall be idempotent. | Event Architecture Baseline; ADR-L5-0010 | Entity version numbers are verified; out-of-order or duplicate events are discarded safely. |
| C-002 | Index schema evolution shall follow blue/green rebuild patterns without search downtime. | Schema Evolution Baseline; ADR-L5-0008 | Full re-indexing occurs in a separate staging index; query traffic swaps atomically via index aliases. |
| C-003 | Failed synchronization events shall be quarantined without halting the global sync pipeline. | Failure Isolation Baseline; ADR-L5-0014 | Poison messages routed to a dead-letter queue; healthy entity indexing continues uninterrupted. |

### 3.3 Assumptions

| ID | Assumption | Why Needed | Impact if False | Validation / Resolution |
|---|---|---|---|---|
| AS-001 | Domain events published via L5-C-02 contain entity IDs and sequence/version numbers. | Enables ordered projection updates. | Out-of-order events overwrite newer data. | Version comparison ensures only monotonic updates apply. |
| AS-002 | Authoritative domain tables support sequential cursor batch scanning for full rebuilds. | Required for disaster recovery re-indexing. | Rebuild requires full table memory dump. | Uses indexed primary key / updated_at pagination in L5-B-02. |

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 Owns

- Search index synchronization event consumers and message dispatchers.
- Transformation pipelines mapping domain events into denormalized search projection documents.
- Index versioning, staging index provisioning, and atomic alias switching.
- Pre-computing access-control security tokens embedded within index documents.
- Full-rebuild batch extractors and progress reconciliation monitors.
- Synchronization dead-letter queues and quarantine management.

### 4.2 Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-B-01-001` | Enterprise Search & Indexing Baseline | Platform collaboration | Informs query engine of active index aliases and freshness timestamps. |
| `L5-B-02-001` | Database & Persistence Architecture Baseline | Source of truth | Scans authoritative persistence tables during full index rebuilds (ADR-L5-0005). |
| `L5-C-02-001` | Event & Messaging Architecture Baseline | Durable events | Subscribes to domain entity lifecycle topics (ADR-L5-0010). |
| `L5-C-02-002` | Durable Event Delivery & Ordering | Message delivery | Consumes durable event streams with guaranteed delivery and consumer group offsets. |
| `L5-D-02-001` | Authorization Architecture Baseline | Security rules | Determines required access-control token structure for entity types (ADR-L5-0009). |
| `L5-A-08-001` | Background Processing & Job Execution | Batch execution | Orchestrates large-scale index rebuild and reconciliation jobs (ADR-L5-0013). |

### 4.3 Must Not Own

- Domain entity state transitions or business event publishing.
- Authoritative persistence schema migration (owned by L5-B-02-004).
- User authentication, credential validation, or session management.
- Processing search query requests or serving client query results (owned by L5-B-01-001).

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| `DomainEventSyncConsumer` | Component | Listens to domain event topics; filters relevant entity events; validates version order. | `DomainChangeEvent` | In-memory offset | Admitted sync task | `L5-C-02-001` | Discards out-of-order or duplicate event payloads. | L4-H-018-BH-001, C-001 |
| `ProjectionDocumentTransformer` | Component | Maps domain event payload into denormalized search document; embeds access tokens. | Admitted sync task | Stateless | Formatted search doc | `L5-D-02-001`, `L5-B-05` | Rejects records with missing mandatory search attributes. | L4-H-018-BH-002 |
| `IndexUpdateDispatcher` | Component | Batches and writes document updates to active search index collection. | Formatted search docs | Active index connection | Index write confirmation | Search backend | Retries on network timeout; routes persistent failures to DLQ. | L4-H-018-BH-003, C-003 |
| `IndexLifecycleManager` | Component | Manages index versions, creates staging indexes, executes schema migrations, performs alias swaps. | Migration / Rebuild command | Index metadata catalog | Updated active alias | `L5-E-04` | Aborts alias swap if staging index validation fails. | C-002, L4-H-019-BH-003 |
| `RebuildReconciliationOrchestrator` | Component | Executes full batch extraction from canonical DB; streams records to staging index. | `IndexRebuildJobRequest` | Job progress state | Completed staging index | `L5-B-02`, `L5-A-08` | Pauses/resumes batch extraction on worker restarts. | L4-H-019-BH-002 |

### 5.2 Primary Technical Flows

#### 5.2.1 Event-Driven Synchronization Flow
```text
Domain Aggregate Mutation (e.g., Student Enrolled, Grade Approved)
         ↓
Domain Event Published to L5-C-02 Topic (via ADR-L5-0007 Outbox)
         ↓
DomainEventSyncConsumer (verify Event Version > Current Index Version)
         ↓
ProjectionDocumentTransformer (denormalize fields, inject D02 Access Tokens)
         ↓
IndexUpdateDispatcher (write document to Active Index Replica)
         ↓
Acknowledge Event Offset to L5-C-02 Broker (Lag metrics recorded)
```

#### 5.2.2 Full Index Rebuild & Recovery Flow (L4-H-019)
```text
Index Corruption Detected OR Schema Migration Triggered
         ↓
IndexLifecycleManager (provision empty Staging Index: "search_student_v2")
         ↓
RebuildReconciliationOrchestrator (paginate authoritative records from L5-B-02)
         ↓
Batch Transform & Bulk Load into Staging Index (via L5-A-08 Work-Units)
         ↓
IndexValidationGate (verify Document Count == DB Record Count; verify Sample Queries)
         ↓
Atomic Alias Swap: alias "search_student_active" -> "search_student_v2"
         ↓
Decommission Obsolete Staging Index ("search_student_v1") after 24h grace period
```

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Out-of-Order Event | `DomainEventSyncConsumer` | Discard event (`LOG_OUT_OF_ORDER_DROPPED`). | Overwriting newer entity state with older historical state. | Consumer continues to next event offset. | C-001, ADR-L5-0010 |
| Malformed Event Payload | `ProjectionDocumentTransformer` | Route event to Quarantine Dead-Letter Queue (`ERR_POISON_EVENT_QUARANTINED`). | Halting the entire consumer thread and blocking subsequent sync. | Operator inspects DLQ; fix transformation schema. | C-003, L4-H-018-BH-004 |
| Index Cluster Write Timeout | `IndexUpdateDispatcher` | Retry with exponential backoff; if exhausted, hold event offset. | Dropping unwritten events silently and creating index drift. | Reconnect to search cluster and replay batch. | ADR-L5-0010, ADR-L5-0014 |
| Staging Index Validation Mismatch | `IndexLifecycleManager` | Abort alias swap (`ERR_REBUILD_VALIDATION_FAILED`); keep existing index. | Swapping active alias to a truncated or corrupted staging index. | Re-run reconciliation job for missing record ranges. | L4-H-019-BH-004 |

### 5.4 Conceptual Contracts

#### `DomainChangeEvent` (Consumed from L5-C-02)
```text
DomainChangeEvent {
  EventId: UUID
  TenantId: UUID
  InstitutionId: UUID
  EntityType: String
  EntityId: UUID
  EntityVersion: Long
  EventType: Enum [CREATED, UPDATED, DELETED, STATUS_CHANGED]
  OccurredAtUtc: Timestamp
  PayloadData: Map<String, Any>
}
```

#### `IndexRebuildJobRequest`
```text
IndexRebuildJobRequest {
  RebuildJobId: UUID
  TenantId: UUID
  TargetEntityType: String
  TargetIndexAlias: String
  NewIndexVersion: String
  BatchChunkSize: Integer (Default: 1000)
  VerifySampleQueryList: List<String>
  DecommissionPriorIndexAfterHours: Integer
}
```

### 5.5 State, Persistence & Consistency Boundaries

Index synchronization relies on **at-least-once message delivery** from `L5-C-02`. Idempotency is enforced by maintaining the highest processed `EntityVersion` per indexed document. Full rebuilds read exclusively from transactionally committed snapshots in `L5-B-02`, guaranteeing that dirty uncommitted database reads never contaminate search indexes.

### 5.6 Asynchronous & Job Realization

- Real-time synchronization runs continuously as asynchronous event consumer daemon workers.
- Full index rebuilds and historical backfills execute as partitioned background batch jobs via `L5-A-08-001` under `ADR-L5-0013`.

### 5.7 Security, Trust & Isolation

- Each projected search document embeds a `SecurityScopeTokens` array containing hashed principal, role, and department tags.
- When `L5-B-01-001` queries the index, it matches caller tokens against this array.
- Multi-tenancy isolation is strictly preserved: index documents inherit the `TenantId` of the originating domain event.

### 5.8 Runtime & Deployment Implications

Synchronization consumers deploy as independent container workers (`L5-E-04-001`), scaled according to incoming message queue depth on `L5-C-02` event topics.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-B01-020` | Idempotent Version-Gated Synchronization | Updates are admitted only if `IncomingEvent.Version >= IndexedDoc.Version`. | C-001; ADR-L5-0010 | Eliminates race conditions and out-of-order event delivery hazards. | Governed by ADR-L5-0010 |
| `AD-B01-021` | Blue/Green Index Alias Swapping | Index schema migrations and full rebuilds execute into parallel staging indexes; switched via atomic alias updates. | C-002; L4-H-019-BH-003 | Ensures 100% search uptime during multi-hour re-indexing jobs. | Governed by ADR-L5-0008 |
| `AD-B01-022` | Embedded Pre-Computed Access Tokens | Access control scopes are pre-computed during ingestion and embedded directly in index records. | L4-H-018-BH-002; ADR-L5-0009 | Avoids expensive cross-service authorization lookups for every search query hit. | Governed by ADR-L5-0009 |
| `AD-B01-023` | Poison Event Quarantine Pipeline | Unparseable or schema-violating events are routed to a dedicated DLQ without stopping consumer streams. | C-003; L4-H-018-BH-004 | Protects synchronization availability across the enterprise when a single event is malformed. | Governed by ADR-L5-0014 |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Synchronization Freshness | Event processing lag targets p99 < 3.0 seconds under normal transactional load. | Operational service level | `L5-B-01-002`, `L5-C-02-001` |
| Zero-Downtime Rebuilds | Blue/green alias swapping ensures search remains responsive during index restructurings. | High availability architecture | `L5-E-01-001` |
| Disaster Recovery RTO | Full index reconstruction from relational persistence completes within defined recovery time objectives. | Business continuity plan | `L5-B-01-002`, `L5-B-02-001` |

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| `CR-001` | Ingestion consumers shall enforce monotonic version gating, dropping older event versions for an entity. | §5.1, `AD-B01-020` | `L4-H-018-BH-001`, C-001 |
| `CR-002` | Projection documents shall embed pre-computed authorization tokens reflecting D02 access boundaries. | §5.1, `AD-B01-022` | `L4-H-018-BH-002`, ADR-L5-0009 |
| `CR-003` | Index rebuilds and schema migrations shall execute in an isolated staging index without disrupting active queries. | §5.2, `AD-B01-021` | `L4-H-019-BH-003`, C-002 |
| `CR-004` | Promotion of a staging index to active query status shall occur atomically via search index alias pointers. | §5.2, `AD-B01-021` | `L4-H-019-BH-004` |
| `CR-005` | The implementation shall verify that staging index record counts reconcile with database counts before promoting an index. | §5.2 | `L4-H-019-BH-004` |
| `CR-006` | Poison or malformed event messages shall be diverted to a quarantine dead-letter queue without crashing the consumer. | §5.3, `AD-B01-023` | `L4-H-018-BH-004`, C-003 |
| `CR-007` | Event consumers shall commit broker offsets only after search index writes are durably confirmed. | §5.2 | ADR-L5-0010 |
| `CR-008` | Full-rebuild batch jobs shall execute as chunked, restartable background work-units via `L5-A-08-001`. | §5.6 | ADR-L5-0013 |
| `CR-009` | The synchronization pipeline shall publish real-time metrics for event consumer lag and projection latency. | §5.1, §5.4 | `L4-H-018-BH-006` |
| `CR-010` | The implementation shall ensure that index document deletions correspond strictly to canonical domain deletion events. | §5.1 | `L4-H-018-BH-002` |
| `CR-011` | Staging index creation shall inherit exact security, replica, and sharding configurations of the production index. | §5.1 | `L5-E-04-001` |
| `CR-012` | Decommissioning of superseded indexes shall enforce a minimum 24-hour retention buffer before physical deletion. | §5.2 | `L4-H-019-BH-001` |
| `CR-013` | Rebuild batch scanning shall read only committed database snapshots, preventing uncommitted dirty reads. | §5.5 | `L5-B-02-001` |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| `OD-001` | Event Broker Consumer Partitioning Scheme | Messaging Integration | Partition key selection (TenantId vs EntityType vs EntityId) | L5-C-02 Owner / Data Architect | No | Standardized to EntityId hash to ensure strict single-entity ordering in Stage 5. |
| `OD-002` | Batch Rebuild Throughput Rate Limiter | Performance / Operations | Maximum database connection pool consumption during re-indexing | L5-E-02 Owner | No | Cap batch queries to 20% of database read-replica IOPS capacity. |
| `OD-003` | Dead-Letter Queue Reprocessing Automated Tooling | Operations Tooling | Automated replay tooling for corrected schema events | Operations Lead | No | Manual review CLI script provisioned in Stage 6. |

---

## 10. Traceability Summary

- `L4-H-018-BH-001` — Realized by `DomainEventSyncConsumer` admitting and filtering domain change events (§5.1, `CR-001`).
- `L4-H-018-BH-002` — Realized by `ProjectionDocumentTransformer` creating non-destructive projection updates (§5.1, `CR-002`, `CR-010`).
- `L4-H-018-BH-003` — Realized by `IndexUpdateDispatcher` updating active index replicas and maintaining freshness (§5.1).
- `L4-H-018-BH-004` — Realized by dead-letter queue quarantine of poison event payloads (§5.3, `CR-006`).
- `L4-H-018-BH-006` — Realized by synchronization offset tracking and event lag metrics (§5.1, `CR-009`).
- `L4-H-018-BH-007` — Realized by projection validation before making documents eligible for query (§5.1).
- `L4-H-019-BH-001` — Realized by `IndexLifecycleManager` isolating corrupted indexes and preserving snapshots (§5.1, `CR-012`).
- `L4-H-019-BH-002` — Realized by `RebuildReconciliationOrchestrator` streaming batch records from database (§5.1, `CR-008`, `CR-013`).
- `L4-H-019-BH-003` — Realized by blue/green staging index provisioning and migration management (§5.1, `CR-003`, `CR-011`).
- `L4-H-019-BH-004` — Realized by atomic index alias swapping following verification (§5.2, `CR-004`, `CR-005`).
