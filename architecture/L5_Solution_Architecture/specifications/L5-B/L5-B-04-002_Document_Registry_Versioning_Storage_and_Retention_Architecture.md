# L5-B-04-002 — Document Registry, Versioning, Storage & Retention Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-B-04-002 |
| Specification Title | Document Registry, Versioning, Storage & Retention Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-B — Data & Information Architecture |
| Major Area | L5-B-04 — Document, Media & Object Platform Architecture |
| Canonical Technical Responsibility | Technical registry, object identity, versions, storage lifecycle, metadata and retention execution. |
| Technical Architecture Owner | Document Registry & Retention Technical Architecture |
| Assigned Author | Varad |
| Version | 1.0.0 |
| Status | Draft |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-B-04-001; L5-B-02-001; L5-B-02-002; L5-B-02-003; L5-B-05-001; L5-B-05-003; L5-D-03-001; L4-C-018; L4-E-064; L4-H-004 |
| Referenced By | L5-B-04-001; legal hold workflows; student/employee records management; compliance archiving |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0002; ADR-L5-0005; ADR-L5-0006; ADR-L5-0014 |

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the solution architecture for the technical Document Registry, version lineage management, physical storage tiering, and retention/disposal execution across SutraERP. It provides immutable version tracking, metadata cataloging, lifecycle state machines, legal hold protections, and automated statutory disposal execution, while strictly ensuring that legal retention policies are governed by `L5-B-05` and business deletion approvals remain with authorized institutional officers.

### In Scope

- Technical Document Registry cataloging all institutional documents, media, and credentials.
- Immutable version lineage trees supporting linear replacement and branching revisions.
- Storage lifecycle management (Hot active tier -> Warm operational archive -> Cold statutory deep archive).
- Enforcement of legal holds freezing document disposal regardless of retention expiry.
- Automated technical disposal execution (cryptographic erasure / secure shredding) upon authorized retention completion.
- Tracking document content hashes (SHA-256), MIME types, byte sizes, and physical storage locators.

### Out of Scope / Non-Goals

- Deciding institutional retention schedules or legal compliance durations (governed by L5-B-05-003).
- Authorizing the destruction or legal hold placement on student or employee records (owned by Compliance Officer).
- Direct physical hardware maintenance of storage server racks or SAN arrays.
- UI/UX version comparison diff viewers or file explorer trees.
- Storing primary relational business entities (student profiles, employee records) in document registry tables.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L4 | `L4-C-018` | Approved / Locked | `L4-C-018-BH-004` | Registration of immutable generated document artifact | Registry entry creation & storage locator binding |
| L4 | `L4-C-018` | Approved / Locked | `L4-C-018-BH-005` | Preserving verifiable certificate identity across student lifecycle | Permanent credential registry tracking |
| L4 | `L4-E-064` | Approved / Locked | `L4-E-064-BH-001` | Registration and technical identity binding for profile documents | Multi-entity registry cataloging |
| L4 | `L4-E-064` | Approved / Locked | `L4-E-064-BH-002` | Version tracking and metadata association for profile attachments | Version lineage tree execution |
| L4 | `L4-E-064` | Approved / Locked | `L4-E-064-BH-003` | Retention lifecycle binding and archival schedule association | Technical retention schedule enforcement |
| L4 | `L4-H-004` | Approved / Locked | `L4-H-004-BH-003` | Registry status update and public verification invalidation | Credential status lifecycle transitions |
| L4 | `L4-H-004` | Approved / Locked | `L4-H-004-BH-005` | Audit trail and evidence capture for credential status history | Version audit history preservation |
| Decision | Corpus Register | Locked | `L5-B-04-002` | Technical registry, versions, storage lifecycle, and retention | Canonical platform responsibility |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | A document under active Legal Hold shall never be purged or deleted, even if retention expires. | L5-B-05-003 Baseline; Statutory Compliance | Legal hold flag overrides automated retention disposal engines unconditionally. |
| C-002 | Stored document versions are strictly immutable (Write-Once-Read-Many). | Regulatory Records Baseline | Updates create a new version record; physical blob overwrites are strictly forbidden. |
| C-003 | Registry metadata must be transactionally persisted in tenant-isolated schemas. | ADR-L5-0005; ADR-L5-0006 | Document registry tables reside in `L5-B-02` tenant persistence. |

### 3.3 Assumptions

| ID | Assumption | Why Needed | Impact if False | Validation / Resolution |
|---|---|---|---|---|
| AS-001 | Retention policies are defined in L5-B-05-003 and bound via RetentionScheduleId. | Separates policy definition from technical execution. | Missing schedule defaults to institutional default retention. | Verified against retention catalog at registration. |
| AS-002 | Deep cold storage supports cryptographic purge verification upon authorized disposal. | Required for GDPR / DPDP compliance. | Manual storage certificate needed. | Confirmed via storage driver erasure hooks. |

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 Owns

- Document Registry database tables, schema models, and indexing.
- Unique document identification (`DocumentId` UUID) and version numbering (`VersionNumber`, `ParentVersionId`).
- Document metadata catalog (mime type, size, SHA-256 hash, creation timestamp, creator principal).
- Multi-tier storage routing (Hot / Warm / Cold) based on age and activity.
- Retention lifecycle scheduler and automated disposal execution workers.
- Legal hold registration, active hold enforcement, and hold release logging.

### 4.2 Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-B-04-001` | Document Platform Baseline | Architecture baseline | Inherits common object identity and storage gateway abstractions. |
| `L5-B-02-001` | Database & Persistence Architecture Baseline | Metadata persistence | Persists registry entries and version trees in relational database (ADR-L5-0005). |
| `L5-B-02-003` | Transaction, Consistency & Concurrency | Transactional integrity | Enforces atomic registry commits and optimistic concurrency locks (ADR-L5-0006). |
| `L5-B-05-003` | Retention, Archival, Deletion & Legal Hold | Policy authority | Consumes statutory retention rules and legal hold placement orders. |
| `L5-D-03-002` | Cryptography & Data Protection | Key management | Executes cryptographic shredding by destroying dedicated document encryption keys. |

### 4.3 Must Not Own

- Defining statutory retention durations (e.g. 7 years for financial records, 50 years for student transcripts).
- Granting authorization to destroy records or lifting legal holds.
- Authoring document contents, rendering templates, or generating PDFs (owned by L5-B-04-004).
- Storing primary academic, personnel, or financial master records.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| `DocumentRegistryService` | Component | Registers new document entities, assigns DocumentId, records metadata, tracks storage locator. | `DocumentRegistrationRequest` | Active registry tables | `RegisteredDocumentEntry` | `L5-B-02-001` | Rejects duplicate registrations or missing content hashes. | L4-C-018-BH-004 |
| `VersionLineageManager` | Component | Manages parent-child version links; creates sequential or branching versions upon document updates. | `DocumentVersionCreateRequest` | Version tree tables | Updated version lineage | `L5-B-02-003` | Enforces optimistic lock on parent version during updates. | C-002, L4-E-064-BH-002 |
| `StorageLifecycleScheduler` | Component | Evaluates document age and access patterns; orchestrates migration from Hot to Warm and Cold tiers. | Storage tiering policies | Tier tracking state | Storage migration task | `L5-A-08`, `L5-B-04-001` | Retries failed blob migration before updating registry locator. | L4-E-064-BH-003 |
| `LegalHoldEnforcer` | Control Point | Enforces legal hold flags; blocks disposal, archival truncation, or physical deletion of held files. | Deletion / Disposal request | Active legal hold catalog | Allowed or Denied decision | `L5-B-05-003` | Strictly denies any deletion command for held documents. | C-001, L4-H-004-BH-005 |
| `DisposalExecutionWorker` | Runtime Component | Executes authorized document destruction via cryptographic shredding and secure blob deletion. | `AuthorizedDisposalBatch` | Stateless worker | `DisposalCertificateRecord` | `L5-D-03-002`, `L5-D-04` | Emits audit certificate proving permanent deletion. | L5-B-05-003 |

### 5.2 Primary Technical Flows

#### 5.2.1 Registration and Version Creation Flow
```text
Document Generator / Ingestion Service
         ↓
DocumentRegistryService (assign Document UUID, validate TenantId, ContentHash)
         ↓
VersionLineageManager (if existing document: link ParentVersionId, increment VersionNumber)
         ↓
L5-B-02 Transaction Commit (insert registry record + version record atomically via ADR-L5-0006)
         ↓
Return RegisteredDocumentEntry to Calling Workflow
```

#### 5.2.2 Retention Expiry & Disposal Flow
```text
Daily Retention Evaluation Cron (L5-A-08)
         ↓
Identify Expired Documents (CurrentDate > RetentionExpiryDate)
         ↓
LegalHoldEnforcer (Verify ActiveLegalHold == FALSE for each candidate)
  [Hold Active] → Skip candidate, log legal hold retention extension
         ↓
Verify Authorized Disposal Order from Compliance Officer
         ↓
DisposalExecutionWorker (invoke L5-D-03 to destroy document envelope encryption key)
         ↓
Physical Storage Deletion (delete blob from storage bucket via StorageAccessGateway)
         ↓
Update Registry Status -> PURGED_DESTROYED (retain metadata tombstone for audit)
         ↓
Emit DisposalCertificateRecord to L5-D-04 Immutable Audit Log
```

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Legal Hold Breach Attempt | `LegalHoldEnforcer` | Immediate command block (`ERR_LEGAL_HOLD_ACTIVE`); emit security alert. | Deleting, truncating, or moving an active legal hold document. | Compliance officer must formally release legal hold. | C-001 |
| Concurrent Version Update Collision | `VersionLineageManager` | Optimistic lock exception (`ERR_CONCURRENT_VERSION_COLLISION`). | Silently overwriting parallel edits on the same parent version. | Client re-fetches latest version and re-submits. | ADR-L5-0006 |
| Storage Migration Blob Failure | `StorageLifecycleScheduler` | Roll back registry locator update (`ERR_MIGRATION_COPY_FAILED`). | Updating database to point to cold storage before blob is copied. | Worker retries copy; preserves hot copy until verified. | ADR-L5-0014 |
| Incomplete Cryptographic Shredding | `DisposalExecutionWorker` | Mark disposal FAILED (`ERR_CRYPTO_PURGE_INCOMPLETE`); alert Security. | Marking document purged while encryption keys remain active. | Security Officer investigates key management interface. | L5-B-05-003 |

### 5.4 Conceptual Contracts

#### `DocumentRegistrationRequest`
```text
DocumentRegistrationRequest {
  TenantId: UUID
  InstitutionId: UUID
  RegistrationCorrelationId: UUID
  OwnerEntityReference: {
    EntityType: String (e.g., "STUDENT", "EMPLOYEE", "COURSE")
    EntityId: UUID
  }
  ContentMimeType: String
  ContentSizeBytes: Long
  ContentSha256Hash: String
  StorageLocatorKey: String
  ClassificationTag: Enum [PUBLIC, INTERNAL, CONFIDENTIAL, RESTRICTED]
  RetentionScheduleId: String
  ParentDocumentId: Optional<UUID> (for new versions)
}
```

#### `RegisteredDocumentEntry`
```text
RegisteredDocumentEntry {
  DocumentId: UUID
  TenantId: UUID
  InstitutionId: UUID
  VersionNumber: Integer
  ParentVersionId: Optional<UUID>
  StorageLocatorKey: String
  CurrentStorageTier: Enum [HOT, WARM, COLD]
  ContentSha256Hash: String
  RetentionExpiryDate: Optional<Timestamp>
  IsLegalHoldActive: Boolean
  LifecycleStatus: Enum [ACTIVE, SUPERSEDED, ARCHIVED, PURGED_DESTROYED]
  RegisteredAtUtc: Timestamp
}
```

### 5.5 State, Persistence & Consistency Boundaries

Registry records reside in `L5-B-02` tenant persistence. The lifecycle transitions are strictly governed:
`ACTIVE` -> `SUPERSEDED` (when newer version created)
`ACTIVE` -> `ARCHIVED` (when moved to cold storage)
`ARCHIVED` -> `PURGED_DESTROYED` (upon verified cryptographic shredding)
Documents under `IsLegalHoldActive == TRUE` can transition between storage tiers for cost optimization, but are blocked from entering `PURGED_DESTROYED`.

### 5.6 Asynchronous & Job Realization

- Storage tiering migrations and bulk retention evaluations execute as scheduled background batch jobs orchestrated via `L5-A-08-001` under `ADR-L5-0013`.
- Disposal execution operates as rate-limited background worker tasks to prevent storage system I/O saturation.

### 5.7 Security, Trust & Isolation

- Document registry entries strictly inherit the `TenantId` of the owner entity.
- Registry tables use tenant-partitioned schemas (`ADR-L5-0005`).
- Cryptographic shredding ensures that even if deleted blobs remain in raw disk blocks, the content is mathematically unrecoverable because the per-document key is destroyed (`L5-D-03-002`).

### 5.8 Runtime & Deployment Implications

Registry services run as high-availability stateless microservices (`L5-E-04-001`) with connection pooling to `L5-B-02` primary database instances.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-B04-020` | Immutable Version Append-Only Tree | Existing document records and physical blobs are never modified in place; updates append new version rows. | C-002; Regulatory integrity | Provides complete audit provenance and allows point-in-time document state reconstruction. | None (Local L5 Architecture Decision) |
| `AD-B04-021` | Hard Legal Hold Preemption Gate | The `LegalHoldEnforcer` control point operates as an unbypassable barrier ahead of all deletion workflows. | C-001; L5-B-05-003 Baseline | Eliminates accidental spoliation of evidence during active legal proceedings or audits. | None (Local L5 Architecture Decision) |
| `AD-B04-022` | Multi-Tier Storage Abstraction | Transparent relocation of document blobs across Hot, Warm, and Cold storage tiers without modifying DocumentId. | Storage optimization baseline | Reduces enterprise infrastructure cost while preserving persistent entity document references. | None (Local L5 Architecture Decision) |
| `AD-B04-023` | Cryptographic Shredding Disposal Model | Document disposal destroys the dedicated per-document symmetric key before deleting raw storage blobs. | L5-B-05-003; Privacy compliance | Guarantees absolute data unrecoverability across distributed storage arrays and backups. | Governed by ADR-L5-0002 |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Legal Hold Compliance | Zero tolerance for spoliation; documents under active hold cannot be destroyed under any circumstance. | Legal regulatory compliance | `L5-B-04-002`, `L5-B-05-003` |
| Storage Cost Optimization | Automatic archival of inactive documents to cold object storage reduces primary storage footprint by >60%. | Operational efficiency | `L5-B-04-002`, `L5-E-02-001` |
| Version Provenance | Complete parent-child version tree ensures every document revision is attributable to an author and timestamp. | Audit & evidence standard | `L5-D-04-003` |

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| `CR-001` | The registry shall record every document with a unique DocumentId UUID, TenantId, and SHA-256 content hash. | §5.1 | `L4-C-018-BH-004` |
| `CR-002` | Modifications to existing documents shall create new version rows with incremented version numbers and parent version pointers. | §5.1, `AD-B04-020` | `L4-E-064-BH-002`, C-002 |
| `CR-003` | Physical document blobs shall never be overwritten in place; new versions shall write to new storage locators. | §5.5, `AD-B04-020` | C-002 |
| `CR-004` | The implementation shall strictly prohibit the deletion or purging of any document flagged with an active legal hold. | §5.1, `AD-B04-021` | C-001, L4-H-004-BH-005 |
| `CR-005` | Storage tier migrations between Hot, Warm, and Cold tiers shall not alter the external DocumentId or content hash. | §5.1, `AD-B04-022` | `L4-E-064-BH-003` |
| `CR-006` | Document disposal shall execute cryptographic shredding by destroying the document encryption key via `L5-D-03-002`. | §5.1, `AD-B04-023` | L5-B-05-003 |
| `CR-007` | Following disposal, the registry shall retain an immutable tombstone record preserving document metadata and disposal audit proof. | §5.2 | `L4-H-004-BH-005` |
| `CR-008` | Registry metadata persistence shall execute within tenant-partitioned database schemas governed by `L5-B-02-001`. | §4.2 | C-003, ADR-L5-0005 |
| `CR-009` | Concurrent version creation on the same parent document shall be protected via optimistic locking. | §5.3 | ADR-L5-0006 |
| `CR-010` | The registry shall track and index documents across Student, Employee, Financial, and Governance domains. | §5.1 | `L4-E-064-BH-001` |
| `CR-011` | Automated retention evaluation and storage tiering shall execute as background jobs via `L5-A-08-001`. | §5.6 | ADR-L5-0013 |
| `CR-012` | Unverified storage blobs failing two-phase registration within 24 hours shall be purged by garbage collection. | §5.3 | `L5-B-04-001` |
| `CR-013` | The registry shall provide query APIs to retrieve the complete historical version tree for any document. | §5.1 | `L4-E-064-BH-002` |
| `CR-014` | The engine shall emit immutable audit events to `L5-D-04` upon every registration, version creation, hold toggle, and purge. | §5.1 | `L4-H-004-BH-005` |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| `OD-001` | Cold Storage Blob Archival Protocol | Storage Integration | S3 Glacier Flexible vs Local Tape Archive integration | Storage Architecture / L5-E-04 | No | Standardize cold storage retrieval latency SLA in Stage 6. |
| `OD-002` | Tombstone Metadata Retention Duration | Policy / Compliance | Duration to retain purged document tombstone rows (Permanent vs 10 Years) | Legal Compliance / L5-B-05 | No | Permanent metadata tombstones mandated for academic credentials. |
| `OD-003` | Automated Deduplication Across Tenants | Security / Architecture | Cross-tenant binary deduplication vs strict tenant isolation | Security Architect / L5-A-04 | No | Cross-tenant deduplication strictly rejected to prevent side-channel leaks. |

---

## 10. Traceability Summary

- `L4-C-018-BH-004` — Realized by `DocumentRegistryService` recording generated student certificate artifacts (§5.1, `CR-001`).
- `L4-C-018-BH-005` — Realized by permanent registry tracking of verifiable student credentials (§5.1, `CR-010`).
- `L4-E-064-BH-001` — Realized by registering employee profile attachments across entity domains (§5.1, `CR-008`).
- `L4-E-064-BH-002` — Realized by `VersionLineageManager` maintaining immutable parent-child version trees (§5.1, `CR-002`, `CR-013`).
- `L4-E-064-BH-003` — Realized by `StorageLifecycleScheduler` managing storage tiering and retention schedules (§5.1, `CR-005`, `CR-011`).
- `L4-H-004-BH-003` — Realized by lifecycle status transitions reflecting credential revocation (§5.5).
- `L4-H-004-BH-005` — Realized by `LegalHoldEnforcer` and `DisposalExecutionWorker` emitting immutable audit records (§5.1, `CR-004`, `CR-014`).
