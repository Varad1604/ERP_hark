# L5-B-04-001 — Document, Media & Object Platform Architecture Baseline

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-B-04-001 |
| Specification Title | Document, Media & Object Platform Architecture Baseline |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-B — Data & Information Architecture |
| Major Area | L5-B-04 — Document, Media & Object Platform Architecture |
| Canonical Technical Responsibility | Common technical document/media/object platform spanning registry, ingestion, storage, generation, verification and credential integrity; no business acceptance/issuance ownership transfer. |
| Technical Architecture Owner | Document & Media Platform Architecture |
| Assigned Author | Varad |
| Version | 1.0.0 |
| Status | Draft |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-B-02-001; L5-B-02-002; L5-B-02-003; L5-D-03-001; L5-D-03-002; L5-B-05-001; L5-B-05-002; L4-C-007; L4-C-018; L4-E-039; L4-E-064; L4-H-002 |
| Referenced By | L5-B-04-002; L5-B-04-003; L5-B-04-004; L5-B-04-005; all domain document and media consumers |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0002; ADR-L5-0005; ADR-L5-0006 |

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the common solution-architecture baseline for managing institutional documents, media files, digital credentials, and binary objects across SutraERP. The platform provides reusable technical services for document registration, storage abstraction, validation, templated rendering, and cryptographic verification, while strictly maintaining that business domains retain sole authority for document eligibility, business acceptance, approval, and official issuance decisions.

### In Scope

- Unified document and media object identity model (canonical Document UUID, Content Hash, Version Lineage).
- Common platform ingestion, storage, and retrieval boundaries across structured and unstructured content.
- Composition of specialized document capabilities (B04-002 through B04-005).
- Tenant-isolated, encrypted object persistence handoff coordinating with `L5-B-02` and `L5-D-03`.
- Technical evidence, audit provenance, and tamper-detection interfaces for document lifecycles.
- Strict boundary preservation preventing the technical platform from assuming business acceptance or approval authority.

### Out of Scope / Non-Goals

- Granting business approval, legal certification, or institutional issuance of certificates (owned by domain workflows).
- Physical object storage appliance or cloud bucket provisioning.
- Cryptographic key custody, certificate authority (CA) root signing, or HSM hardware management (owned by L5-D-03).
- UI/UX document preview viewers, drag-and-drop file upload widgets, or form layouts (owned by L6).
- Direct modification of student or employee relational records from document operations.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L4 | `L4-C-007` | Approved / Locked | `L4-C-007-BH-002` | Admission document submission and technical intake | Common document intake and registration baseline |
| L4 | `L4-C-007` | Approved / Locked | `L4-C-007-BH-003` | Technical file verification and structural assessment | Verification handoff baseline |
| L4 | `L4-C-018` | Approved / Locked | `L4-C-018-BH-003` | Approved certificate request to technical generation handoff | Generation initiation baseline |
| L4 | `L4-C-018` | Approved / Locked | `L4-C-018-BH-004` | Execution of generation into immutable document artifact | Generated artifact storage baseline |
| L4 | `L4-C-018` | Approved / Locked | `L4-C-018-BH-005` | Application of digital signature, seal, and QR code | Credential integrity orchestration baseline |
| L4 | `L4-E-039` | Approved / Locked | `L4-E-039-BH-003` | Employee certificate generation to official document outcome | Employee document realization baseline |
| L4 | `L4-E-064` | Approved / Locked | `L4-E-064-BH-001` | Employee profile document intake and registration | Profile document lifecycle baseline |
| L4 | `L4-H-002` | Approved / Locked | `L4-H-002-BH-002` | Credential package signing and verifiable storage | External compliance document baseline |
| Decision | Corpus Register | Locked | `L5-B-04-001` | Reusable document platform; no business acceptance ownership | Canonical platform responsibility |
| Decision | Stage-4 Proceed | Locked | Stage-4 Gate | Mandates minimum dependencies: `B02 + D03 + B05` | Dependency-gate boundary enforcement |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | Business domains retain sole authority for document acceptance, eligibility, and approval. | L5 Candidate Register §3.1 | Platform operations carry technical status only (e.g., `VALIDATED`, `GENERATED`); never business approval. |
| C-002 | Binary content and document metadata must be stored with cryptographic content addressing. | Security & Storage Baseline | Every stored object is indexed by immutable SHA-256 digest; content changes require new version UUIDs. |
| C-003 | Cryptographic keys, secrets, and signing primitives remain strictly under L5-D-03 ownership. | ADR-L5-0002; L5-D-03 Baseline | B04 orchestrates document signing workflows but delegates cryptographic execution to D03. |
| C-004 | Document metadata persistence must enforce tenant database isolation under L5-B-02. | ADR-L5-0005 | Document registry tables reside within tenant-isolated persistence schemas. |

### 3.3 Assumptions

| ID | Assumption | Why Needed | Impact if False | Validation / Resolution |
|---|---|---|---|---|
| AS-001 | Underlying storage backends provide immutable blob write capabilities (WORM / S3 object lock). | Required for tamper-proof regulatory retention. | Emulated via application-level access control. | Validated through storage adapter configurations. |
| AS-002 | Concrete storage driver selection (Ceph vs MinIO vs S3) is technology-neutral at solution level. | Preserves on-premises and cloud portability. | Storage drivers adhere to uniform S3-compatible interface. | Governed by Open Decision OD-001. |

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 Owns

- Common document platform abstraction layer coordinating registry, storage, ingestion, and generation.
- Universal document and object identity model (`DocumentId`, `ContentHash`, `StorageLocator`).
- Metadata schema standards for document classifications, retention policies, and access scopes.
- High-level orchestration of specialized document capabilities (`L5-B-04-002` through `L5-B-04-005`).
- Unified technical document error taxonomy and audit evidence emission.

### 4.2 Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-B-02-001` | Database & Persistence Architecture Baseline | Persistence handoff | Stores document registry metadata, version histories, and retention tables (ADR-L5-0005). |
| `L5-B-02-003` | Transaction, Consistency & Concurrency | Transactional boundary | Enforces atomic commit between domain entity state and document registration (ADR-L5-0006). |
| `L5-D-03-001` | Canonical Trust Boundary & Segmentation | Security boundary | Enforces access segmentation for protected document storage networks (ADR-L5-0001). |
| `L5-D-03-002` | Cryptography & Data Protection | Encryption & Hashing | Encrypts document blobs at rest and generates SHA-256 integrity digests (ADR-L5-0002). |
| `L5-B-05-001` | Data Classification & Retention Baseline | Policy context | Applies classification tags (Public, Internal, Confidential, Restricted) to documents. |
| `L5-B-05-002` | Sensitive Data Handling & Masking | Privacy protection | Sanitizes and redacts sensitive PII from document logs and previews. |
| `L5-B-04-002..005` | Specialized Document Platform Specifications | Platform specialization | Delegates registry, ingestion, generation, and signing to specialized sibling specifications. |

### 4.3 Must Not Own

- Business rules regarding student admission eligibility, scholarship qualification, or degree clearance.
- Deciding whether an uploaded identity proof or certificate is legally acceptable.
- Cryptographic private key custody or certificate revocation list management (owned by L5-D-03).
- Direct mutation of student, employee, or financial database aggregates.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| `DocumentPlatformCoordinator` | Control Point | Admits document platform requests, verifies tenant scope, and routes to specialized engine. | `DocumentPlatformRequest` | Stateless | Admitted platform task | `L5-A-04`, `L5-D-01` | Rejects missing tenant or unauthenticated requests. | L4-C-018-BH-003 |
| `ObjectIdentityManager` | Component | Generates canonical Document UUID, computes SHA-256 content hash, resolves version lineage. | Binary stream & metadata | Stateless | `DocumentObjectDescriptor` | `L5-D-03-002` | Rejects payloads failing integrity hash check. | C-002, L4-E-064-BH-002 |
| `SpecializedCapabilityRouter` | Component | Dispatches tasks to specialized B04 engines: Registry (002), Ingestion (003), Gen (004), Signing (005). | Admitted platform task | Stateless | Delegated execution result | `L5-B-04-002..005` | Handles sub-engine timeouts or processing faults. | Candidate Register §3.1 |
| `StorageAccessGateway` | Component | Provides unified S3-compatible blob storage abstraction; applies envelope encryption. | Blob stream & encryption key | Ephemeral buffer | Encrypted storage locator | `L5-D-03-002`, `L5-B-02` | Fails on storage volume exhaustion or network disconnect. | ADR-L5-0002, C-004 |
| `PlatformAuditEmitter` | Component | Emits tamper-evident lifecycle audit events for document creation, access, modification, and deletion. | Document lifecycle event | Stateless | `DocumentAuditEventRecord` | `L5-B-02`, `L5-D-04` | Guarantees audit log persistence before returning response. | L4-C-018-BH-004 |

### 5.2 Primary Technical Flow

```text
Domain Workflow (e.g., L4-C-018 Certificate Request)
         ↓
DocumentPlatformCoordinator (verify Tenant, Authenticate Principal, Check Rate Limits)
         ↓
ObjectIdentityManager (generate Document UUID, compute SHA-256 Digest)
         ↓
SpecializedCapabilityRouter (delegate to specialized sub-engine):
   ├── L5-B-04-003 Ingestion (validate MIME, scan viruses, verify integrity)
   ├── L5-B-04-004 Generation (bind template, render PDF artifact)
   └── L5-B-04-005 Signing (apply digital signature, visual seal, signed QR)
         ↓
StorageAccessGateway (encrypt blob via L5-D-03, write to Tenant Storage Partition)
         ↓
DocumentRegistryManager (persist metadata, locator, hash in L5-B-02)
         ↓
PlatformAuditEmitter (emit immutable audit record via L5-D-04)
         ↓
Return DocumentObjectDescriptor to Calling Domain Workflow (Status: READY)
```

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Content Hash Corruption | `ObjectIdentityManager` | Reject document (`ERR_CONTENT_INTEGRITY_MISMATCH`). | Storing corrupted or truncated binary streams. | Client must re-upload file stream. | C-002, ADR-L5-0002 |
| Storage Target Outage | `StorageAccessGateway` | Circuit break and retry with backoff (`ERR_STORAGE_UNAVAILABLE`). | Dropping file upload or returning fake success. | Retry transient connection; failover to secondary pool. | ADR-L5-0014 |
| Encryption Envelope Fault | `StorageAccessGateway` | Abort storage write (`ERR_ENCRYPTION_FAILURE`). | Storing sensitive documents in unencrypted plaintext. | Re-acquire cryptographic key context from L5-D-03. | C-003, ADR-L5-0002 |
| Registry Persistence Failure | `DocumentRegistryManager` | Roll back stored blob write (`ERR_REGISTRY_WRITE_FAILED`). | Orphaned storage blobs without database registry entries. | Garbage collection worker cleans uncommitted blobs. | ADR-L5-0005, ADR-L5-0006 |

### 5.4 Conceptual Contracts

#### `DocumentPlatformRequest`
```text
DocumentPlatformRequest {
  TenantId: UUID
  InstitutionId: UUID
  PlatformCorrelationId: UUID
  CallingPrincipalId: UUID
  ActionType: Enum [INGEST_VALIDATE, GENERATE_RENDER, SIGN_SEAL, RETRIEVE, ARCHIVE]
  TargetDocumentId: Optional<UUID>
  PayloadParameters: Map<String, Any>
}
```

#### `DocumentObjectDescriptor`
```text
DocumentObjectDescriptor {
  DocumentId: UUID
  TenantId: UUID
  InstitutionId: UUID
  DocumentVersion: Integer
  ContentMimeType: String (e.g., "application/pdf", "image/png")
  ContentSizeBytes: Long
  ContentSha256Hash: String (Hex-encoded)
  StorageLocatorKey: String (Opaque blob storage reference)
  ClassificationTag: Enum [PUBLIC, INTERNAL, CONFIDENTIAL, RESTRICTED]
  RetentionScheduleId: String
  DigitalSignatureStatus: Enum [UNSIGNED, SIGNED_VERIFIED, SIGNATURE_FAILED]
  TechnicalStatus: Enum [INGESTED, VALIDATED, GENERATED, QUARANTINED, DECOMMISSIONED]
  CreatedAtUtc: Timestamp
}
```

### 5.5 State, Persistence & Consistency Boundaries

Document binary payloads are stored in an **immutable, content-addressed object store**. Metadata, version lineages, and retention schedules are stored in relational persistence governed by `L5-B-02` under `ADR-L5-0005`. Atomic consistency between binary storage writes and database metadata records is achieved via a two-phase registration protocol: blobs are written in `PENDING` state and transitioned to `ACTIVE` only upon successful database transaction commit (`ADR-L5-0006`).

### 5.6 Asynchronous & Job Realization

Heavy file processing operations (virus scanning large archives, bulk PDF certificate rendering, mass archival transfers) are orchestrated asynchronously via `L5-A-08-001` under `ADR-L5-0013`. Real-time document retrieval and metadata lookups operate synchronously.

### 5.7 Security, Trust & Isolation

1. **Storage-Level Encryption:** Every document blob is encrypted at rest using envelope encryption keys managed by `L5-D-03-002`.
2. **Access URL Tokenization:** Direct public URLs to storage buckets are strictly prohibited; retrieval URLs are pre-signed, time-bounded (max 15 minutes), and scoped to the authenticated principal.
3. **Tenant Blob Isolation:** Storage buckets enforce logical folder prefix or dedicated bucket partitioning per Tenant ID (`ADR-L5-0005`).

### 5.8 Runtime & Deployment Implications

Document gateway workers deploy as containerized stateless runtimes (`L5-E-04-001`), utilizing fast local ephemeral scratch storage for streaming transformations, decoupled from core business API pods.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-B04-001` | Content-Addressed Immutability | Document blobs are identified and verified by their cryptographic SHA-256 content digest; mutation creates a new version. | C-002; Regulatory integrity | Prevents silent document tampering, deduplicates binary content, and ensures exact audit verification. | Governed by ADR-L5-0002 |
| `AD-B04-002` | Separation of Platform from Business Authority | The document platform produces technical status only; official acceptance/issuance remains with calling domain. | Corpus Register L5-B-04-001; C-001 | Preserves domain authority boundaries; document engine cannot grant degrees or approve admissions. | Governed by ADR-L5-0005 |
| `AD-B04-003` | Opaque Storage Locator Indirection | Domain entities reference documents only via canonical `DocumentId` UUIDs, never direct storage bucket URLs. | Architecture isolation baseline | Decouples business aggregates from physical storage topology and allows seamless storage migrations. | None (Local L5 Architecture Decision) |
| `AD-B04-004` | Two-Phase Blob Registration | Binary upload writes to `PENDING` state; metadata commits to `L5-B-02` before promoting blob to `ACTIVE`. | ADR-L5-0006; Data consistency | Eliminates orphaned storage objects and prevents dangling database foreign keys. | Governed by ADR-L5-0006 |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Blob Encryption at Rest | All stored objects encrypted using AES-256-GCM envelope encryption keys from L5-D-03. | Data protection standard | `L5-D-03-002`, `L5-B-04-001` |
| Tenant Storage Isolation | Tenant storage is strictly isolated via path prefixing and tenant-scoped credentials. | Multi-tenant security | `L5-A-04-001`, `L5-D-03-005` |
| Tamper-Proof Lineage | SHA-256 content hashes are captured in tamper-evident audit streams under L5-D-04. | Legal non-repudiation | `L5-D-04-003` |

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| `CR-001` | The platform shall admit document operations only after validating caller TenantId and InstitutionId context. | §5.1 | `L4-C-018-BH-003` |
| `CR-002` | Every document stored on the platform shall possess a verified SHA-256 content hash matching its binary stream. | §5.1, `AD-B04-001` | C-002 |
| `CR-003` | Binary document blobs shall be encrypted at rest using cryptographic keys managed under `L5-D-03-002`. | §5.7, `AD-B04-004` | C-003, ADR-L5-0002 |
| `CR-004` | The platform shall return technical status only (`VALIDATED`, `GENERATED`) and shall not issue official business approvals. | §4.3, `AD-B04-002` | C-001 |
| `CR-005` | Direct public storage bucket URLs are forbidden; document retrieval shall utilize time-bounded pre-signed URLs. | §5.7 | ADR-L5-0001 |
| `CR-006` | Document metadata, versions, and locators shall be persisted transactionally in tenant-isolated `L5-B-02` schemas. | §5.5, `AD-B04-004` | C-004, ADR-L5-0005 |
| `CR-007` | Uploads failing virus scanning or structural validation shall be quarantined immediately. | §5.1, §5.3 | `L4-C-007-BH-004` |
| `CR-008` | Domain aggregate records shall reference documents exclusively through canonical `DocumentId` UUIDs. | §5.1, `AD-B04-003` | `L4-E-064-BH-001` |
| `CR-009` | Modifications to existing documents shall create an explicit new version without overwriting the previous version. | §5.5, `AD-B04-001` | `L4-E-064-BH-002` |
| `CR-010` | The implementation shall emit immutable audit events covering document creation, access, and deletion via `L5-D-04`. | §5.1, §5.4 | `L4-C-018-BH-004` |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| `OD-001` | Physical Object Storage Provider Driver | ADR-Required | MinIO vs Ceph RADOS vs AWS S3 SDK | Lead Architect / L5-B-04 | No | Evaluate storage adapter portability in Stage 6 consolidation. |
| `OD-002` | Automated Storage Tiering Thresholds | Policy / Operations | Hot to cold storage transition duration (e.g. 90 days) | Storage Administrator | No | Configured via institutional storage lifecycle policies in L5-B-04-002. |
| `OD-003` | Maximum Streaming Upload Buffer Size | Performance / Network | Maximum synchronous upload payload (50MB vs 100MB) | L5-E-02 Owner | No | Payloads > 50MB mandated to use multi-part chunked uploads. |

---

## 10. Traceability Summary

- `L4-C-007-BH-002` — Realized by `DocumentPlatformCoordinator` admitting uploaded applicant verification files (§5.1, `CR-001`).
- `L4-C-007-BH-003` — Realized by structural assessment and content validation handoff to L5-B-04-003 (§5.1, `CR-007`).
- `L4-C-018-BH-003` — Realized by generation routing handoff from approved student certificate request (§5.1, `CR-004`).
- `L4-C-018-BH-004` — Realized by `StorageAccessGateway` and `DocumentRegistryManager` persisting generated PDF artifacts (§5.1, `CR-002`, `CR-006`).
- `L4-C-018-BH-005` — Realized by credential integrity handoff to L5-B-04-005 for digital signing and seal (§5.1).
- `L4-E-039-BH-003` — Realized by official employee certificate storage and descriptor generation (§5.1, `CR-008`).
- `L4-E-064-BH-001` — Realized by `ObjectIdentityManager` registering employee profile attachments (§5.1, `CR-008`).
- `L4-E-064-BH-002` — Realized by version lineage tracking preserving historical employee profile uploads (§5.1, `CR-009`).
- `L4-H-002-BH-002` — Realized by verifiable credential package storage and cryptographic registration (§5.1, `CR-003`).
