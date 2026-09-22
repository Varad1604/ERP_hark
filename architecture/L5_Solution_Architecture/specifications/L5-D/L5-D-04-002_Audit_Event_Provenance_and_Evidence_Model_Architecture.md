# L5-D-04-002 — Audit Event, Provenance & Evidence Model Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-04-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-04 — Audit & Evidence Architecture  
**Canonical Technical Responsibility:** Define the reusable technical provenance and evidence model for audit events, including actor attribution, technical source attribution, causality, correlation and evidence relationships, while preserving source-domain ownership of business meaning, business rules, canonical events and institutional authority.  
**Technical Architecture Owner:** L5-D-04-002 — Audit Event, Provenance & Evidence Model Architecture  
**Assigned Author:** Varad  
**Version:** `1.1.2`  
**Status:** `Locked`  
**Last Updated:** `2026-09-22`  
**Planned Production Path:** `architecture/L5_Solution_Architecture/specifications/L5-D/L5-D-04-002_Audit_Event_Provenance_and_Evidence_Model_Architecture.md`  
**Depends On:** `L5_Governance_and_Architecture_Standard.md`; `L5_Validation_Standard.md`; `L5_Solution_Architecture_Specification_Template_v1.0.md`; `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md`; `L5_Dependency_Gated_Parallel_Authoring_Execution_Timeline_2026-09-09.md`; `L5-D-04-001`; `L5-D-03-001`; `L5-A-04-001`; `L5-A-04-002`; `L5-B-02-003`; `L4-A-022`; `L4-A-023`; `L4-A-024`; `ADR-L5-0001`; `ADR-L5-0004`; `ADR-L5-0007`.  
**Referenced By:** `L5-D-04-001`; `L5-D-04-003`; cross-cutting application, workflow, security, integration, and computation specifications.  
**Applicable ADRs:** `ADR-L5-0001` — Canonical Trust Boundary & Segmentation Pattern; `ADR-L5-0004` — Tenant and Institution Isolation Enforcement Pattern; `ADR-L5-0007` — Atomic State-to-Publication Intent Handoff Pattern.  
**Boundary Correction Authority:** `L5_D04-002_Ownership_Boundary_Correction_and_Relock_Decision_2026-09-13.md`.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the canonical technical data structures, conceptual contracts, and provenance relationship models used across SutraERP to represent audit evidence. It establishes how actor attribution, technical execution identities, causal linkages, workflow correlation, and entity delta descriptors are structured into a standardized, immutable evidence model. This model enables verifiable historical reconstruction, investigation querying, and cryptographic protection without absorbing business domain entities or altering canonical business events.

### 2.2 In Scope

- Canonical conceptual contracts for audit evidence: `AuditEvidenceEnvelope`, `ActorAttributionContext`, `EntityChangeRecord`, and `CausalityLink`.
- Multi-tier actor attribution model distinguishing human users, system accounts, service workloads, and delegation chains.
- Explicit distinction between direct causality (directed acyclic graph linkages) and shared correlation (grouping flow identifiers).
- Reference-based entity modification representation (capturing logical change descriptors and digests without duplicating source-domain aggregates).
- Standards for recording ambiguous, incomplete, or broken provenance without fabricating synthetic facts.
- Integration contracts consumed by `L5-D-04-001` (Capture Baseline) and handed off to `L5-D-04-003` (Tamper-Evident Storage).

### 2.3 Out of Scope / Non-Goals

- Shared capture boundaries, runtime interceptors, and buffer queues (owned by `L5-D-04-001`).
- Physical database DDL, physical persistence schema, storage-engine selection, transaction primitives and physical persistence topology (owned by `L5-B-02`).
- Cryptographic algorithm selection and cryptographic protection operations (owned by `L5-D-03-002`); cryptographic key custody/lifecycle remains `L5-D-03-003`-owned.
- Audit-specific integrity-stream, sequential-linkage, checkpoint, anchor, verification-result persistence and audit-specific evidence/proof semantics (owned by `L5-D-04-003`).
- Reusable export archive/file packaging, transport, large-file transfer and generic delivery-integrity mechanics (owned by `L5-C-03-008`).
- Business domain entity schemas, master data truth, or lifecycle definitions (owned by originating business domains).
- Authentication mechanisms, session management, or credential verification (owned by `L5-D-01`).
- Authorization role definitions, policy rules, or permission evaluation (owned by `L5-D-02`).
- Creation or modification of canonical business events (owned by originating business domains).
- Hardcoding specific serialization formats (e.g., Protobuf, Avro, JSON) without governed ADR approval.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Realization / Contribution |
|---|---|---|---|---|---|
| L4 | `L4-A-022` | v1.0.0 / Draft — explicitly authorized as behavioral authority for this controlled L5 validation scope (see OD-002) | `L4-A-022-BH-003` | Attributes accountable actor using authoritative identity, actor-type, delegation, auth references. | Realized by `ActorAttributionResolver` producing canonical `ActorAttributionContext` (`AD-001`). |
| L4 | `L4-A-022` | v1.0.0 / Draft — see OD-002 | `L4-A-022-BH-004` | Records affected business-entity references and logical changes without duplicating full aggregates. | Realized by `EntityChangeDescriptor` producing `EntityChangeRecord` (`AD-003`, `CR-003`). |
| L4 | `L4-A-022` | v1.0.0 / Draft — see OD-002 | `L4-A-022-BH-005` | Associates authoritative correlation identifier and at most one direct causation reference. | Realized by `CorrelationContextBinder` and `CausalityGraphBuilder` enforcing at most one parent (`AD-002`). |
| L4 | `L4-A-022` | v1.0.0 / Draft — see OD-002 | `L4-A-022-BH-010` | Preserves immutable evidence identity, source event, context, actor, changes, correlation, causation. | Realized by `AuditEvidenceEnvelope` schema in Section 5.4 (`CR-001`). |
| L4 | `L4-A-023` | v1.0.0 / Draft — see OD-002 | `L4-A-023-BH-004` | Resolves required correlation and direct causation relationships without modifying canonical meaning. | Realized by `CausalityGraphBuilder` query resolution model (`AD-002`). |
| L4 | `L4-A-023` | v1.0.0 / Draft — see OD-002 | `L4-A-023-BH-005` | Assembles immutable investigation Evidence Package referencing canonical evidence. | Realized by `AuditEvidenceEnvelope` reference structure (`CR-006`). |
| L4 | `L4-A-024` | v1.0.0 / Draft — see OD-002 | `L4-A-024-BH-002` | Verifies evidence identity, immutability, timestamp, actor attribution, execution context. | Realized by `LineageIntegrityGuard` verifying envelope integrity fields (`CR-007`). |
| L4 | `L4-A-024` | v1.0.0 / Draft — see OD-002 | `L4-A-024-BH-004` | Verifies correlation, direct causation, and historical continuity without rewriting records. | Realized by `LineageIntegrityGuard` acyclic graph validation (`CR-008`). |
| ADR | `ADR-L5-0001` | Architecture Approved | Trust Boundary Pattern | Explicit trust boundary admission; context preservation. | Enforced in `ActorAttributionContext` and `AuditEvidenceEnvelope` security metadata. |
| ADR | `ADR-L5-0004` | Architecture Approved | Tenant Isolation Pattern | Mandatory Tenant/Institution scoping and partition isolation. | Strict requirement in `AuditEvidenceEnvelope.tenant_scope` (`CR-002`). |
| ADR | `ADR-L5-0007` | Architecture Approved | Atomic Intent Handoff | Transactional handoff of state changes and intent. | Realized in `EntityChangeRecord` digest alignment with domain outbox state. |

### 3.2 Architectural Constraints

- **C-001 (Actor Disambiguation):** Actor attribution shall explicitly distinguish between the human principal initiating the action, the machine principal executing the request, and any intermediate delegation chain (`L4-A-022-BH-003`, `AD-001`).
- **C-002 (Causality vs. Correlation Separation):** Direct causation (identifying the specific predecessor event that triggered execution) must never be conflated with shared correlation (identifying a broad business process or transaction flow) (`L4-A-022-BH-005`, `AD-002`).
- **C-003 (Anti-Duplication Constraint):** Entity change models shall capture only entity identifiers, change classifications, field masks, and cryptographic digests. Cloning entire domain entity records into audit envelopes is strictly prohibited (`L4-A-022-BH-004`, `AD-003`).
- **C-004 (Explicit Ambiguity):** If actor identity, correlation, or causation cannot be determined with certainty, the model shall record explicit `UNKNOWN` or `AMBIGUOUS` markers; synthesizing artificial facts is prohibited (`AD-004`).
- **C-005 (Canonical Event Immutability):** Canonical business events are referenced by immutable event identifier and digest; the evidence model shall not republish or modify source event contracts (`L4-A-022-BH-010`, `AD-005`).

### 3.3 Assumptions

- **AS-001:** `L5-D-04-001` provides the runtime capture boundaries, context extraction, and buffer pipelines that populate the contracts defined herein.
- **AS-002:** `L5-D-04-003` consumes the `AuditEvidenceEnvelope` defined herein to realize audit-specific deterministic integrity commitments, sequential integrity linkage, integrity streams/epochs, checkpoints/anchors, verification and proof semantics. `L5-B-02` remains owner of physical persistence/schema/transaction realization and `L5-D-03` remains owner of cryptographic protection operations and key custody.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 Owns

- Canonical data models and conceptual contracts for audit evidence: `AuditEvidenceEnvelope`, `ActorAttributionContext`, `EntityChangeRecord`, and `CausalityLink`.
- Technical attribution hierarchy and delegation chain representation.
- Directed acyclic graph (DAG) rules and validation semantics for direct causality.
- Correlation identifier propagation and binding semantics.
- Representation of partial, incomplete, or ambiguous provenance chains.
- Structural integrity validation rules for audit records and relationship links.

### 4.2 Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary Description | Why Needed |
|---|---|---|---|
| `L5-D-04-001` | Audit Baseline | Supplies runtime context and raw observation events. | Populates and validates `AuditEvidenceEnvelope` instances at execution boundaries. |
| `L5-D-04-003` | Tamper-Evident Storage | Hands off populated `AuditEvidenceEnvelope` records for audit-specific integrity protection, verification, proof and durable canonical-evidence semantics. | Protects conceptual evidence through D04-003-owned integrity-stream/checkpoint/anchor semantics while physical persistence stays B02-owned and cryptographic operations stay D03-owned. |
| `L5-D-01-002` | Identity Account & Service Identity | Provides authenticated identity models (Person, Account, ServicePrincipal). | Binds authoritative identity structures into `ActorAttributionContext`. |
| `L5-D-02-005` | Delegation Architecture | Provides time-bounded delegation records and authority chains. | Represents acting-on-behalf-of relationships without inventing delegation authority. |
| `L5-A-04-002` | Tenant Context Propagation | Provides validated Tenant and Institution context tokens. | Guarantees strict multi-tenant boundary binding within evidence envelopes. |
| `L5-B-02-003` | Persistence Baseline | Supplies entity reference conventions and optimistic concurrency state digests. | Aligns `EntityChangeRecord` with domain persistence models under `ADR-L5-0007`. |

### 4.3 Must Not Own

- Runtime evidence capture interceptors or buffer queues (owned by `L5-D-04-001`).
- Physical database schemas, storage-engine selection, transaction mechanics or physical persistence topology (owned by `L5-B-02`).
- Cryptographic algorithms/protection operations or key custody (owned by `L5-D-03-002/003`).
- Audit-specific integrity-stream/checkpoint/anchor and evidence/proof persistence semantics (owned by `L5-D-04-003`).
- Reusable export archive/file packaging, transport or delivery-integrity mechanics (owned by `L5-C-03-008`).
- Business domain state, entities, or rules (owned by source domains).
- Authentication mechanisms or identity account lifecycle (owned by `L5-D-01`).
- Authorization decisions or policy evaluation (owned by `L5-D-02`).
- Canonical business events or event publication schedules (owned by source domains).

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| `ActorAttributionResolver` | Component | Resolves composite actor context from ambient security tokens; separates human, machine, and delegation layers. | Security token, delegation context. | Stateless resolution. | Validated `ActorAttributionContext`. | `L5-D-01`, `L5-D-02-005` | Mark `AMBIGUOUS_ACTOR` if unresolvable. | `L4-A-022-BH-003`, `AD-001` |
| `CorrelationContextBinder` | Component | Extracts or initializes flow correlation identifier; preserves correlation across thread/async boundaries. | Request header, incoming message metadata. | Distributed correlation context. | Bound `correlation_id` string. | `L5-C-01`, `L5-C-02` | Generate new root correlation ID. | `L4-A-022-BH-005`, `AD-002` |
| `CausalityGraphBuilder` | Component | Identifies direct causal predecessor; validates acyclic graph property; binds at most one causal parent. | Triggering event ID, parent evidence ID. | Local causal graph edge. | Validated `CausalityLink`. | `L5-D-04-001` | Mark `ROOT_CAUSE` or reject on cycle. | `L4-A-022-BH-005`, `AD-002` |
| `EntityChangeDescriptor` | Component | Generates logical delta descriptors for affected business entities without copying full aggregate state. | Domain entity reference, pre/post mutation digests. | Change descriptor structure. | Validated `EntityChangeRecord`. | `L5-B-02`, `ADR-L5-0007` | Fallback to entity ID reference only. | `L4-A-022-BH-004`, `AD-003` |
| `EvidenceEnvelopeFactory` | Component | Assembles validated components into the canonical `AuditEvidenceEnvelope` aggregate. | Enriched contexts, payload digest, timestamps. | Assembled envelope. | Hand off to `L5-D-04-001` ingestion pipeline. | `L5-D-04-001`, `L5-A-04` | Reject on missing mandatory fields. | `L4-A-022-BH-010`, `CR-001` |
| `LineageIntegrityGuard` | Control Point | Verifies structural integrity, immutability, and acyclic continuity of provenance graphs during audit queries. | Audit query results, investigation scope. | Traversed provenance DAG. | Integrity verification findings or validated lineage. | `L5-D-04-003` | Surface `BROKEN_LINEAGE` finding. | `L4-A-024-BH-002`, `BH-004` |

### 5.2 Primary Technical Flow

```text
[Runtime Execution Boundary]
             │
             ├──► [ActorAttributionResolver] ───► Resolves Human Actor, Machine Identity & Delegation Chain
             │
             ├──► [CorrelationContextBinder] ────► Resolves Cross-Cutting Correlation Identifier
             │
             ├──► [CausalityGraphBuilder] ───────► Identifies & Validates Direct Predecessor Link (DAG)
             │
             └──► [EntityChangeDescriptor] ─────► Extracts Entity Reference, Delta Type & Digests
                             │
                             ▼
                [EvidenceEnvelopeFactory]
                             │
                             ▼ Assembles Canonical Aggregate
                  [AuditEvidenceEnvelope]
                             │
                             ▼
                [Handoff to L5-D-04-001 / L5-D-04-003]
```

1. **Attribution Resolution:** The `ActorAttributionResolver` inspects execution credentials, distinguishing the human subject, executing service principal, and active delegation authorization.
2. **Context & Correlation Binding:** The `CorrelationContextBinder` preserves the end-to-end trace correlation ID. The `CausalityGraphBuilder` identifies the immediate triggering event and confirms it forms an acyclic causal step.
3. **Delta Extraction:** The `EntityChangeDescriptor` records the affected entity type, entity ID, operation classification (e.g., `CREATED`, `STATE_TRANSITION`), and cryptographic state digests without copying internal business aggregate fields.
4. **Envelope Assembly:** The `EvidenceEnvelopeFactory` combines these components into an immutable `AuditEvidenceEnvelope`, verifies all mandatory metadata fields, and passes the aggregate to the ingestion and storage pipeline.

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Unresolvable Actor Identity | `ActorAttributionResolver` | Mark actor attribution as `UNRESOLVED_SUBJECT`; record machine execution principal. | Fabricate synthetic human identity or assign default admin user. | Flag as security finding; log via `L5-D-04-001`. | `AD-001`, `AD-004` |
| Causal Loop Detected | `CausalityGraphBuilder` | Reject causal parent assignment; record causality as `CYCLE_DETECTED`; record direct predecessor as broken link. | Create cyclic causality graph or freeze execution pipeline. | Surface verification failure in `L4-A-024`. | `L4-A-024-BH-004`, `CR-008` |
| Entity Aggregate Duplication | `EntityChangeDescriptor` | Enforce field mask and digest extraction; reject payloads containing full domain entity snapshots. | Persist complete business aggregates inside audit evidence envelope. | Strip non-permitted payload fields. | `L4-A-022-BH-004`, `AD-003` |
| Correlation Context Loss | `CorrelationContextBinder` | Generate new root correlation ID; record `CORRELATION_DISCONTINUITY` flag in envelope. | Silently inherit unrelated correlation ID or drop correlation field. | Operational diagnostic alert via `L5-E-05`. | `AD-002`, `CR-004` |
| Broken Provenance Linkage | `LineageIntegrityGuard` | Return verified evidence with explicit `MISSING_PREDECESSOR` finding; do not block query. | Fabricate missing link or claim complete historical verification. | Governed verification finding via `L4-A-024-BH-005`. Note: `LineageIntegrityGuard` applies this failure path exclusively during `L4-A-024` integrity verification query context, not during initial `AuditEvidenceEnvelope` assembly. | `L4-A-024-BH-005`, `AD-004` |

### 5.4 Canonical Conceptual Contracts

#### Contract 1: `AuditEvidenceEnvelope` (Top-Level Canonical Aggregate)

```text
AuditEvidenceEnvelope {
  evidence_id: UUID
  evidence_version: String                // e.g., "1.0.0"
  tenant_scope: {
    tenant_id: UUID
    institution_id: UUID
  }
  source_context: {
    source_component: String              // e.g., "StudentBillingService"
    source_runtime_node: String
    channel: String                       // e.g., "WEB_UI", "API", "BATCH_WORKER"
    session_id: String?
  }
  business_event_reference: {
    event_id: UUID
    event_type: String                    // e.g., "StudentChargePublished"
    event_digest: String                  // Cryptographic digest of original event payload (algorithm selection governed by L5-D-03-002)
  }
  actor_attribution: ActorAttributionContext
  causality: CausalityLink?
  correlation_id: String
  affected_entities: List<EntityChangeRecord>
  classification: {
    audit_level: String                   // e.g., "CRITICAL_FINANCIAL", "STATUTORY"
    retention_policy_ref: String
  }
  evidence_payload_digest: String         // Cryptographic digest of normalized payload (algorithm selection governed by L5-D-03-002)
  timestamp_epoch_ms: Long
}
```

#### Contract 2: `ActorAttributionContext` (Multi-Tier Attribution)

```text
ActorAttributionContext {
  subject_type: HUMAN | SERVICE_PRINCIPAL | SYSTEM_WORKER | ANONYMOUS
  authenticated_subject_id: String        // Canonical Person ID or Workload Identity
  acting_service_id: String               // Service executing the technical operation
  delegation: {
    is_delegated: Boolean
    delegator_subject_id: String?
    delegation_authorization_ref: String? // Points to L5-D-02-005 approved delegation
    delegation_scope: String?
  }?
  security_context_ref: String            // Points to session token or auth decision ID
}
```

#### Contract 3: `EntityChangeRecord` (Reference-Based Mutation Context)

```text
EntityChangeRecord {
  entity_type: String                     // e.g., "StudentChargeObligation"
  entity_id: String
  domain_owner: String                    // e.g., "Finance"
  change_type: CREATED | MODIFIED | STATE_TRANSITION | CANCELLED | PURGED
  affected_field_names: List<String>      // Field mask: names of fields mutated
  pre_state_digest: String?               // Hash of pre-mutation entity state
  post_state_digest: String               // Hash of post-mutation entity state
}
```

#### Contract 4: `CausalityLink` (Direct Antecedent Linkage)

```text
CausalityLink {
  predecessor_evidence_id: UUID
  causation_type: DIRECT_INVOCATION | EVENT_REACTION | WORKFLOW_STEP | MANUAL_OVERRIDE
  causal_depth: Integer                   // Incremented depth from root cause (1, 2, ...)
}
```

### 5.5 State, Persistence & Consistency

- **Contract Immutability:** Once an `AuditEvidenceEnvelope` is assembled, its fields are frozen. `L5-D-04-003` defines deterministic version-bound integrity commitment, sequential-linkage, checkpoint/anchor and proof semantics over that immutable envelope; cryptographic protection operations are invoked through `L5-D-03-002`, while physical persistence/schema/transaction realization remains `L5-B-02`-owned. No concrete Merkle construction or storage engine is mandated by this specification.
- **Reference Alignment with `L5-B-02`:** `EntityChangeRecord` uses entity digests compatible with optimistic concurrency control fields established by `ADR-L5-0006` (consumed via `L5-B-02-003` — not a direct ADR applicability for `L5-D-04-002`) and transactional outbox boundaries defined under `ADR-L5-0007`.

### 5.6 Security, Privacy & Sensitive Data

- **PII and Sensitive Data Minimization:** In accordance with `L5-B-05` (Privacy & Sensitive Data), `EntityChangeRecord` records field masks and cryptographic digests rather than plain-text sensitive attributes (e.g., Aadhaar numbers, personal bank details).
- **Scope Segregation:** In accordance with `ADR-L5-0004`, every envelope strictly incorporates `tenant_scope`. Provenance graph traversal across Tenant boundaries is blocked at the model layer.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-001` | Multi-Tier Actor Attribution Hierarchy | Structure actor context into distinct, explicit layers: Authenticated Subject, Executing Service, and Delegation Chain. | `L4-A-022-BH-003`, `L5-D-01` | Prevents conflating human user responsibility with machine service accounts or automated workflow orchestrators. | None (Core Model) — ADR necessity evaluated: the three-layer attribution structure is derived directly from the multi-tier identity obligation in `L4-A-022-BH-003`. No cross-family technology selection is made; ADR not required. |
| `AD-002` | Strict Separation of Causality and Correlation | Model causality as a directed acyclic graph (at most one immediate predecessor) and correlation as a shared flow identifier. | `L4-A-022-BH-005`, `L4-A-023-BH-004` | Correlation groups events within an end-to-end business journey, while causality strictly proves which exact step initiated another. Conflating them destroys root-cause auditability. | None (Core Model) — ADR necessity evaluated: the DAG causality model is a direct structural derivation of the `L4-A-022-BH-005` behavioral obligation to record at most one direct causation reference. No alternative pattern was viable without violating BH semantics; ADR not required. |
| `AD-003` | Reference-Based Entity Delta Model | Represent business entity modifications via entity references, change types, field masks, and cryptographic digests without copying full aggregate bodies. | `L4-A-022-BH-004`, `HF-04` | Copying full domain entities into audit records bloats storage, violates single-source-of-truth ownership, and creates stale duplicate business state. | None (Core Model) — ADR necessity evaluated: the reference-only model is mandated by `L4-A-022-BH-004` (no full aggregate duplication) and `HF-04` (no ownership transfer). No technology choice is made; ADR not required. |
| `AD-004` | Explicit Ambiguity Representation | Represent missing, incomplete, or ambiguous provenance explicitly using standardized enum states rather than synthesizing inferred relationships. | `L4-A-024-BH-005`, `C-004` | An audit system must never hallucinate certainty. Provenance uncertainty is an authoritative investigative finding that must be preserved. | None (Core Model) — ADR necessity evaluated: explicit ambiguity recording is a direct realization of `L4-A-024-BH-005` (classify defects, do not repair). No cross-family pattern or technology choice; ADR not required. |
| `AD-005` | Source Business Event Reference Immutability | Evidence envelopes bind source business events via immutable event ID and payload digest; they do not publish competing business events or modify source schemas. | `L4-A-022-BH-010`, `HF-03` | Preserves source domain ownership over canonical event schemas and prevents audit infrastructure from becoming an unauthorized event publisher. | None (Core Model) — ADR necessity evaluated: this is a direct application of the ownership preservation constraint in `L4-A-022-BH-010` and `HF-03`. No technology selection is made; ADR not required. |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| **Tenant Isolation** | All envelope models mandate validated Tenant and Institution context fields. | `ADR-L5-0004` | `L5-A-04-001` |
| **Data Minimization** | Entity change models avoid raw PII storage; rely on digests and field masks. | `L5-B-05` | `L5-B-05-001` |
| **Attribution Non-Repudiation** | Actor context binds authentication decisions and delegation references. | `L4-A-022-BH-003` | `L5-D-01-002`, `L5-D-02-005` |
| **Causal Graph Determinism** | Causal links strictly enforce acyclic directed graph invariants. | `L4-A-024-BH-004` | `L5-D-04-002` |
| **Historical Integrity** | Evidence envelopes are frozen aggregates designed for D04-003-owned audit integrity protection while physical persistence and cryptographic operations remain with B02 and D03 respectively. | `L4-A-022-BH-010` | `L5-D-04-003`, `L5-B-02`, `L5-D-03` |
| **Audit Query Performance** | Correlation ID indexing enables fast reconstruction without deep graph traversal. | `AD-002` | `L5-D-04-003`, `L5-B-02` |
| **Interoperability** | Model expressed as technology-neutral conceptual contracts prior to serialization. | Repository Standard | `L5-D-04-002` |

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| `CR-001` | Audit evidence shall be modeled using the canonical `AuditEvidenceEnvelope` conceptual contract. | Section 5.4 | `L4-A-022-BH-010` |
| `CR-002` | Every `AuditEvidenceEnvelope` shall include validated Tenant and Institution scope identifiers. | Section 5.4 | `ADR-L5-0004`, `C-004` |
| `CR-003` | Entity modification records shall not duplicate complete source domain aggregates; they shall record references, field masks, and digests. | `AD-003` | `L4-A-022-BH-004`, `C-003` |
| `CR-004` | Actor attribution shall explicitly distinguish human subjects, executing machine identities, and delegation chains. | `AD-001` | `L4-A-022-BH-003`, `C-001` |
| `CR-005` | Direct causality links shall point to at most one immediate predecessor evidence aggregate. | `AD-002` | `L4-A-022-BH-005` |
| `CR-006` | Causality relationships shall form a directed acyclic graph; cyclic causality is strictly forbidden. | Section 5.1 | `L4-A-024-BH-004` |
| `CR-007` | Investigation packages shall reference canonical evidence aggregates by ID and digest without creating duplicate records. | Section 5.4 | `L4-A-023-BH-005` |
| `CR-008` | Missing or ambiguous provenance elements shall be represented explicitly using standardized `UNKNOWN` or `AMBIGUOUS` markers. | `AD-004` | `L4-A-024-BH-005`, `C-004` |
| `CR-009` | Evidence models shall reference source canonical business events by immutable identifier and digest without modifying the event. | `AD-005` | `L4-A-022-BH-010`, `C-005` |
| `CR-010` | Provenance models shall not encode plain-text passwords, private cryptographic keys, or machine secrets. | Section 5.6 | `ADR-L5-0001`, `L5-D-03-001` |
| `CR-011` | Structural validation of evidence envelopes shall be executable prior to dispatch to D04-003-owned audit integrity processing and B02-owned physical persistence realization. | Section 5.1 | `L4-A-024-BH-002` |
| `CR-012` | The model shall not mandate a proprietary wire format or storage engine without governed ADR approval. | Section 2.3 | `L5_Technology_Selection_and_ADR_Methodology.md` |
| `CR-013` | Delegated actions shall preserve the delegator identity and delegation authorization reference. | `AD-001` | `L4-A-022-BH-003`, `L5-D-02-005` |
| `CR-014` | The model shall maintain distinct identifiers for distributed trace correlation versus immediate causal invocation. | `AD-002` | `L4-A-022-BH-005`, `L4-A-023-BH-004` |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| `OD-001` | Exact L4 BH Applicability | Upstream Traceability | None — Resolved against canonical `L4-A-022`, `L4-A-023`, and `L4-A-024`. | Lead Architect | **No (Closed)** | Mapped into Section 3.1 and Section 10. |
| `OD-002` | L4 Upstream Status | Upstream Authority | `L4-A-022`, `L4-A-023`, `L4-A-024` carry Status: Draft in the repository. Explicitly authorized as behavioral authority for this controlled L5 validation scope; the L4 files remain Draft and are not promoted by this L5 decision. | Lead Architect | **No (Conditionally Closed — pending L4 governance approval or explicit Lead Architect authorization referencing this specification)** | Update §3.1 status and this row once L4 governance is resolved. |
| `OD-003` | Physical Envelope Serialization Encoding | Technology / ADR | Concrete serialization format (Protobuf, JSON-LD, Avro) for storage handoff. | D04 / B02 / D03 ADR Governance | No | D04-003 requires a deterministic version-bound integrity representation but does not select a serialization product/format; resolve only if a concrete choice becomes materially architectural. |
| `OD-004` | Provenance Graph Query Language | Technology / Architecture | Selection of graph traversal mechanisms for complex multi-hop investigation queries. | Persistence / Query Architecture | No | To be addressed during query read-model specialization without transferring evidence-model ownership. |

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-022-BH-003` — Accountable actor attribution across identity, type, and delegation (`AD-001`, `CR-004`, `CR-013`).
- `L4-A-022-BH-004` — Recording affected entity references and logical changes without aggregate cloning (`AD-003`, `CR-003`).
- `L4-A-022-BH-005` — Binding correlation ID and at most one direct causation link (`AD-002`, `CR-005`, `CR-014`).
- `L4-A-022-BH-010` — Preserving immutable evidence identity, source event, context, actor, and changes (`CR-001`, `CR-009`).
- `L4-A-023-BH-004` — Resolving correlation and direct causation relationships during investigation (`AD-002`, `CR-014`).
- `L4-A-023-BH-005` — Assembling immutable investigation Evidence Packages by reference (`CR-007`).
- `L4-A-024-BH-002` — Verifying evidence identity, timestamps, actor attribution, and execution context (`CR-011`).
- `L4-A-024-BH-004` — Verifying correlation, direct causation, and historical continuity (`CR-006`).
- `L4-A-024-BH-005` — Identifying missing/broken evidence links without fabricating synthetic data (`AD-004`, `CR-008`).

### Material Higher-Level & ADR Constraints

- `ADR-L5-0001` — Explicit trust boundary admission; context preservation in evidence envelopes (`CR-010`).
- `ADR-L5-0004` — Strict Tenant/Institution scoping and partition isolation (`CR-002`).
- `ADR-L5-0007` — Alignment with atomic state-to-publication handoff and entity state digests (`AD-003`).

### Core L5 Dependencies

- `L5-D-04-001` — Consumes runtime observation events; exports assembled `AuditEvidenceEnvelope`.
- `L5-D-04-003` — Consumes conceptual evidence contracts for audit-specific integrity-stream, checkpoint/anchor, verification and evidence/proof semantics; physical persistence remains B02-owned and cryptographic protection remains D03-owned.
- `L5-D-01-002` — Consumes authenticated identity models for actor attribution.
- `L5-D-02-005` — Consumes delegation models for actor attribution.
- `L5-B-02-003` — Aligns entity change digests with persistence models.

### Architecture Decisions

- `AD-001` (Multi-Tier Actor Attribution Hierarchy)
- `AD-002` (Strict Separation of Causality and Correlation)
- `AD-003` (Reference-Based Entity Delta Model)
- `AD-004` (Explicit Ambiguity Representation)
- `AD-005` (Source Business Event Reference Immutability)

### Conformance Requirements

- `CR-001` through `CR-014`


## 2026-09-22 Batch 7B Impact Revalidation Synchronization

Non-material repository synchronization only. Current dependency metadata and/or authority wording is aligned to canonical governance without changing business authority, technical ownership, behavioral semantics, conformance requirements, traceability relation state, or lifecycle approval/lock basis.
