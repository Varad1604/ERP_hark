# L5-D-04-001 — Audit & Evidence Architecture Baseline

## 1. Specification Metadata

**Specification ID:** `L5-D-04-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-04 — Audit & Evidence Architecture  
**Canonical Technical Responsibility:** Define the shared technical audit and evidence capability used to capture, qualify, correlate and make available evidence of material system behavior, while source domains retain ownership of business meaning, business evidence requirements and institutional authority.  
**Technical Architecture Owner:** L5-D-04-001 — Audit & Evidence Architecture Baseline  
**Assigned Author:** Varad  
**Version:** `1.1.0`  
**Status:** `Review`  
**Last Updated:** `2026-09-12`  
**Planned Production Path:** `architecture/L5_Solution_Architecture/specifications/L5-D/L5-D-04-001_Audit_and_Evidence_Architecture_Baseline.md`  
**Depends On:** `L5_Governance_and_Architecture_Standard.md`; `L5_Validation_Standard.md`; `L5_Solution_Architecture_Specification_Template_v1.0.md`; `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md`; `L5_Dependency_Gated_Parallel_Authoring_Execution_Timeline_2026-09-09.md`; `L5-D-03-001`; `L5-A-04-001`; `L5-A-04-002`; `L5-B-02-003`; `L4-A-022`; `L4-A-023`; `ADR-L5-0001`; `ADR-L5-0004`; `ADR-L5-0007`.  
**Referenced By:** `L5-D-04-002`; `L5-D-04-003`; cross-cutting application, workflow, integration, security and computational engine specifications.  
**Applicable ADRs:** `ADR-L5-0001` — Canonical Trust Boundary & Segmentation Pattern; `ADR-L5-0004` — Tenant and Institution Isolation Enforcement Pattern; `ADR-L5-0007` — Atomic State-to-Publication Intent Handoff Pattern.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the foundational technical architecture for capturing, qualifying, enriching, buffering, and retrieving audit evidence across SutraERP. It establishes the reusable technical infrastructure required to ensure that observable system behavior—ranging from state-changing business facts to security-critical policy decisions—generates immutable, context-rich technical evidence without transferring business authority or record ownership away from originating domains.

### 2.2 In Scope

- Reusable evidence capture boundaries across synchronous application requests, asynchronous messaging, and computational engines.
- Evidence qualification separating mandatory conformance-critical evidence from advisory operational telemetry.
- Architectural realization of dual-path ingestion: synchronous transactional capture versus asynchronous bounded buffering.
- Technical context enrichment binding Tenant, Institution, Session, Workflow, Channel, and Actor identities.
- Coordination with atomic state-to-publication handoffs under `ADR-L5-0007`.
- Fail-closed containment mechanisms for mandatory evidence failures.
- Controlled read-only retrieval gateway architecture for investigations and integrity verification.
- Enforcing non-exposure of cryptographic keys, machine credentials, and authentication secrets in audit streams.

### 2.3 Out of Scope / Non-Goals

- Detailed data schemas and provenance models for audit events and causality links (owned by `L5-D-04-002`).
- Physical storage implementation, cryptographic sealing, tamper-evident hash chaining, and evidence export packaging (owned by `L5-D-04-003`).
- Business domain audit policy, compliance rules, or legal hold definitions (owned by originating business domains).
- Creation or modification of canonical business events or domain lifecycles.
- Authentication mechanisms and user account lifecycle (owned by `L5-D-01`).
- Authorization policy decision logic and permission evaluation (owned by `L5-D-02`).
- Selection of physical storage products, database engines, or third-party log brokers without governed ADR treatment.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Realization / Contribution |
|---|---|---|---|---|---|
| L4 | `L4-A-022` | v1.0.0 / Draft — treated as Locked behavioral authority under governance program assumption (see OD-002) | `L4-A-022-BH-001` | Precondition: source event must be canonical, completed fact, and audit-eligible. | Realized by `AuditEvidenceAdmissionFilter` evaluating eligibility before ingestion (`AD-001`). |
| L4 | `L4-A-022` | v1.0.0 / Draft — see OD-002 | `L4-A-022-BH-002` | Resolves authoritative Tenant, Institution, workflow, session, channel, security references. | Realized by `AuditContextEnricher` binding context without re-owning upstream facts (`CR-003`). |
| L4 | `L4-A-022` | v1.0.0 / Draft — see OD-002 | `L4-A-022-BH-006` | Applies governed Audit Classification without transferring policy ownership from Authorization. | Realized by `AuditEvidenceAdmissionFilter` consuming `L5-D-02` classification policy. |
| L4 | `L4-A-022` | v1.0.0 / Draft — see OD-002 | `L4-A-022-BH-007` | Creates one immutable Audit Evidence aggregate and prevents duplicate creation. | Realized by `AuditIngestionPipeline` with deduplication and append-only constraints (`AD-002`). |
| L4 | `L4-A-022` | v1.0.0 / Draft — see OD-002 | `L4-A-022-BH-008` | Publishes `Audit Evidence Generated` event after complete evidence creation. | Realized by `AuditPublicationHandoff` using `ADR-L5-0007` atomic outbox mechanics (`CR-007`). |
| L4 | `L4-A-022` | v1.0.0 / Draft — see OD-002 | `L4-A-022-BH-009` | Makes immutable evidence available for authorized investigation and eligible timeline projection without altering domain facts. | Retrieval availability realized by `AuditRetrievalGateway` read-only query boundaries (`CR-008`). Timeline Projection read-model availability and materialization owned by `L5-D-04-003` — see §10 disposition note and MF-02 resolution. |
| L4 | `L4-A-023` | v1.0.0 / Draft — see OD-002 | `L4-A-023-BH-001` | Investigation retrieval requires validated requester identity, authorization, and scope. | Realized by `AuditRetrievalGateway` admission checks against `L5-D-01`/`L5-D-02`. |
| L4 | `L4-A-023` | v1.0.0 / Draft — see OD-002 | `L4-A-023-BH-002` | Locates canonical Audit Evidence by reference within authorized scope without cloning. | Realized by `AuditRetrievalGateway` scoped query resolution (§5.1 `AuditRetrievalGateway`, `CR-008`, `ADR-L5-0004`). |
| L4 | `L4-A-023` | v1.0.0 / Draft — see OD-002 | `L4-A-023-BH-007` | Constructs and provides read-only Investigation View preserving domain facts. | Realized by `AuditRetrievalGateway` read model integration. |
| ADR | `ADR-L5-0001` | Architecture Approved | Trust Boundary Pattern | Explicit logical trust boundaries; non-transitive trust; governed admission. | Directs `EvidenceCaptureInterceptor` and `AuditRetrievalGateway` boundary controls. |
| ADR | `ADR-L5-0004` | Architecture Approved | Tenant Isolation Pattern | Mandatory Tenant/Institution scope validation; no cross-tenant data leakage. | Enforced by `AuditContextEnricher` and scoped partition retrieval (`CR-004`). |
| ADR | `ADR-L5-0007` | Architecture Approved | Atomic Intent Handoff | Transactional state change and publication intent committed atomically. | Directs `MandatoryEvidenceGuard` and `AuditPublicationHandoff` integration with `L5-B-02`. |
| L5 Peer | `L5-D-03-001` | Architecture Approved | §4.2, §7.1 | Non-exposure of credentials, keys, and tokens across system boundaries. | Enforced by `AuditEvidenceAdmissionFilter` redaction rules (`AD-005`). |

### 3.2 Authoritative Constraints

- **C-001 (Fail-Closed Mandatory Evidence):** Where authoritative upstream behavior makes evidence generation a completion condition, technical failure of evidence capture shall fail the primary operation closed (`L4-A-022-BH-001`, `AD-003`).
- **C-002 (Immutability):** Generated Audit Evidence aggregates are append-only. Mutation, in-place updates, and revision overwrites are architecturally prohibited (`L4-A-022-BH-007`, `AD-002`).
- **C-003 (Credential Redaction):** Secret keys, passwords, bearer tokens, machine private credentials, and raw authentication secrets shall be stripped or rejected at the evidence capture boundary (`ADR-L5-0001`, `AD-005`).
- **C-004 (Scope Confinement):** Evidence context shall strictly bind Tenant and Institution identifiers; cross-tenant correlation or query aggregation without governed boundary authority is prohibited (`ADR-L5-0004`).
- **C-005 (Business Ownership Invariance):** The audit capability records technical observations of business occurrences; it does not approve, modify, post, or own business transactions (`HF-03`, `HF-04`).
- **C-006 (Technology Neutrality):** No database product, streaming engine, or hardware appliance is mandated without an approved repository ADR (`L5_Technology_Selection_and_ADR_Methodology.md`).

### 3.3 Assumptions

- **AS-001:** Detailed provenance graphs, actor attribution models, and causality schemas are authored in peer specification `L5-D-04-002`.
- **AS-002:** Physical tamper-evident storage structures, cryptographic seal chaining, and legal export packaging are authored by Dhruv in Stage Three specification `L5-D-04-003`.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 Owns

- Shared evidence capture boundary interfaces and interceptors across SutraERP execution runtimes.
- Dual-path evidence ingestion architecture (synchronous mandatory path vs. asynchronous advisory queue).
- Technical context enrichment (Tenant, Institution, Session, Channel, Trace/Correlation ID binding).
- Conformance-critical evidence completion gates and fail-closed transaction guards.
- Deduplication and idempotency verification for incoming evidence events.
- Read-only evidence retrieval gateway and authorization handoff for investigations.

### 4.2 Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary Description | Why Needed |
|---|---|---|---|
| `L5-D-04-002` | Provenance & Evidence Model | Consumes conceptual contracts (`AuditEvidenceEnvelope`, `CausalityLink`). | Structures captured evidence payloads into standardized technical models. |
| `L5-D-04-003` | Tamper-Evident Storage & Export | Hands off validated evidence aggregates for cryptographic chaining and immutable storage. | Dispatches evidence to tamper-resistant persistence without re-owning storage topology. |
| `L5-D-01-001` | IAM Architecture Baseline | Consumes authenticated user and workload identity tokens. | Extracts actor attribution facts without owning user credentials. |
| `L5-D-02-001` | Authorization Baseline | Evaluates retrieval access policies and audit classification metadata. | Controls evidence access without creating authorization rules. |
| `L5-D-03-001` | Security & Trust Baseline | Consumes trust boundary admission rules and credential redaction standards. | Guarantees evidence streams remain free of secrets and keys. |
| `L5-A-04-001` | Tenant & Institutional Isolation | Consumes validated Tenant/Institution context tokens. | Guarantees complete logical isolation across multi-tenant audit records. |
| `L5-B-02-003` | Database & Persistence | Realizes local transactional outbox persistence for mandatory audit records. | Implements atomic commit of state and evidence intent under `ADR-L5-0007`. |
| `L5-C-02-001` | Events & Messaging | Transports asynchronous advisory evidence and dispatches `Audit Evidence Generated`. | Distributes audit events without making messaging authoritative over audit truth. |

### 4.3 Must Not Own

- Detailed provenance and causality data models (owned by `L5-D-04-002`).
- Tamper-evident cryptographic storage, Merkle tree sealing, and export archives (owned by `L5-D-04-003`).
- Business domain transaction records, accounting entries, or academic transcripts (owned by originating business domains).
- Authentication credential lifecycle, token issuance, or password policies (owned by `L5-D-01`).
- Authorization role definitions, policy authoring, or permission evaluation (owned by `L5-D-02`).
- Operational metrics, telemetry dashboards, or system APM monitoring (owned by `L5-E-05`).

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| `EvidenceCaptureInterceptor` | Control Point | Intercepts operations at runtime execution boundaries (API, service, computational engine). | Invocation context, request payload, security header. | Ephemeral execution state. | Normalized raw observation to `AuditEvidenceAdmissionFilter`. | `L5-A-04-001`, `L5-D-03-001` | Local operation boundary. | `L4-A-022-BH-001`, `AD-001` |
| `AuditEvidenceAdmissionFilter` | Component | Validates completed-fact status, checks audit eligibility, redacts prohibited secrets/keys. | Raw observation, classification policy. | Stateless rule evaluation. | Admitted evidence candidate or drop event. | `L5-D-02`, `L5-D-03-001` | Drop or fail admission. | `L4-A-022-BH-001`, `AD-005` |
| `AuditContextEnricher` | Component | Binds authoritative Tenant, Institution, Session, Channel, Correlation, and Causation context. | Admitted candidate, ambient security token. | Enriched evidence envelope. | Forward to `MandatoryEvidenceGuard` or `AdvisoryEvidenceBuffer`. | `L5-A-04`, `L5-D-01`, `L5-D-04-002` | Reject if Tenant context unresolvable. | `L4-A-022-BH-002`, `ADR-L5-0004` |
| `MandatoryEvidenceGuard` | Control Point | Enforces transactional consistency for conformance-critical evidence; coordinates atomic outbox commit. | Enriched mandatory evidence, transaction handle. | Uncommitted outbox record. | Atomic commit with business state or abort transaction. | `L5-B-02-003`, `ADR-L5-0007` | Fail-closed: aborts host business transaction. | `L4-A-022-BH-007`, `AD-003` |
| `AdvisoryEvidenceBuffer` | Buffer / Store | Staging queue for non-critical, operational, or advisory evidence events. | Enriched advisory evidence. | Bounded memory/disk ring buffer. | Async dispatch to `AuditIngestionPipeline`. | `L5-E-01` | Drop-oldest with telemetry alert on saturation. | `AD-001`, `CR-005` |
| `AuditIngestionPipeline` | Service | Deduplicates events, formats into canonical `AuditEvidenceEnvelope`, coordinates persistence handoff. | Staged or outbox evidence items. | Deduplication cache (fingerprint / event ID). | Handoff to `L5-D-04-003` storage and `AuditPublicationHandoff`. | `L5-D-04-002`, `L5-D-04-003` | Retry or dead-letter queue. | `L4-A-022-BH-007`, `AD-002` |
| `AuditPublicationHandoff` | Adapter | Emits canonical notification `Audit Evidence Generated` once storage is guaranteed. | Persisted evidence ID, domain context. | Outbox publication intent. | Dispatches event to `L5-C-02`. | `L5-C-02`, `ADR-L5-0007` | Durable message retry. | `L4-A-022-BH-008` |
| `AuditRetrievalGateway` | Gateway | Governed read-only query boundary for investigations and integrity evaluations. Does not produce or expose Timeline Projection read models — that responsibility is owned by `L5-D-04-003`. | Query parameters, requester token, scope. | Read-only projection queries. | Scoped evidence records or authorization denial. | `L5-D-02`, `L5-A-04`, `L5-D-04-003` | Deny on invalid scope/auth. | `L4-A-023-BH-001`, `L4-A-023-BH-002` |

### 5.2 Primary Technical Flow

```text
[System Behavior / Trigger]
          │
          ▼
[EvidenceCaptureInterceptor] ────────► Extracts execution context & raw observation
          │
          ▼
[AuditEvidenceAdmissionFilter] ──────► Evaluates completed-fact & redacts secrets
          │
          ▼
[AuditContextEnricher] ─────────────► Resolves & binds Tenant, Institution, Session, Actor
          │
          ├────────────────────────────────────────┬────────────────────────────────────────┐
          ▼ (Mandatory Conformance Path)           ▼ (Advisory / Telemetry Path)            ▼ (Eligibility Rejection)
[MandatoryEvidenceGuard]                  [AdvisoryEvidenceBuffer]               [Drop with Log]
          │ (Atomic Commit with Host State)        │ (Bounded Asynchronous Queue)
          │                                        │
          └───────────────────┬────────────────────┘
                              ▼
                  [AuditIngestionPipeline]
                              │
                  ┌───────────┴───────────┐
                  ▼                       ▼
      [Handoff to L5-D-04-003]  [AuditPublicationHandoff]
      (Tamper-Evident Storage)  (Emits "Audit Evidence Generated" via L5-C-02)
```

1. **Interception & Admission:** The `EvidenceCaptureInterceptor` captures runtime execution events at service, transaction, or computational engine boundaries. The `AuditEvidenceAdmissionFilter` confirms the event represents a completed action and strips any raw credentials or secrets.
2. **Context Enrichment:** The `AuditContextEnricher` extracts validated Tenant, Institution, Session, and Actor tokens, appending them to the evidence context.
3. **Dual-Path Routing:**
   - **Mandatory Path:** For conformance-critical business and security actions, the `MandatoryEvidenceGuard` writes the evidence intent into the local persistence outbox within the same database transaction as the business state change (`ADR-L5-0007`). If evidence write fails, the entire transaction rolls back.
   - **Advisory Path:** For non-blocking operational observations, the event enters the bounded `AdvisoryEvidenceBuffer`.
4. **Ingestion & Storage Handoff:** The `AuditIngestionPipeline` verifies deduplication fingerprints, constructs the canonical `AuditEvidenceEnvelope` defined by `L5-D-04-002`, and dispatches the record to `L5-D-04-003` for cryptographic sealing and tamper-evident storage.
5. **Notification:** Once evidence storage is confirmed, `AuditPublicationHandoff` releases the canonical event `Audit Evidence Generated` to the event broker (`L5-C-02`).

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Mandatory Evidence Write Failure | `MandatoryEvidenceGuard` | Abort and rollback host business transaction; surface explicit technical error. | Silently commit business state without required audit evidence. | Transaction rollback via `L5-B-02-003`. | `L4-A-022-BH-001`, `AD-003` |
| Advisory Buffer Saturation | `AdvisoryEvidenceBuffer` | Drop oldest advisory item; record buffer drop counter metric; continue processing. | Block host application threads or allocate unbounded heap memory. | Operational telemetry alert via `L5-E-05`. | `AD-001`, `CR-005` |
| Prohibited Secret Ingestion Attempt | `AuditEvidenceAdmissionFilter` | Redact sensitive field; log security violation event; admit sanitized payload. | Record unencrypted passwords, API tokens, or private keys in evidence store. | Security containment via `L5-D-03-001`. | `AD-005`, `ADR-L5-0001` |
| Tenant Context Unresolvable | `AuditContextEnricher` | Reject evidence creation; reject host operation if mandatory. | Record orphan audit record with default or null Tenant identifier. | Fail-closed validation error. | `ADR-L5-0004`, `CR-004` |
| Storage Handoff Failure | `AuditIngestionPipeline` | Retain record in persistent outbox; schedule exponential backoff retry. | Discard uncommitted evidence or claim successful completion to caller. | Persistent retry queue via `L5-B-02` / `L5-D-04-003`. | `L4-A-022-BH-007` |
| Unauthorized Retrieval Attempt | `AuditRetrievalGateway` | Reject request with access denial; record security audit event of failed attempt. | Disclose evidence across Tenant or unpermitted classification boundaries. | Authorization denial via `L5-D-02`. | `L4-A-023-BH-001` |

### 5.4 Conceptual Contracts & Boundary Information

Detailed evidence models and causality contracts are canonically owned by `L5-D-04-002`. At this baseline boundary, the shared capability requires:

```text
AuditObservationRequest {
  observation_id: UUID
  source_component: String
  event_type: String
  qualification_category: MANDATORY_CONFORMANCE | ADVISORY_OPERATIONAL
  tenant_scope: TenantContextReference
  actor_attribution: ActorAttributionReference
  payload_descriptor: LogicalChangeDescriptor
  correlation_context: CorrelationIdentifierReference
  causation_context: CausationIdentifierReference?
  timestamp_epoch_ms: Long
}
```

### 5.5 State, Persistence & Consistency Boundaries

1. **Dual-Path Persistence Realization:**
   - **Path A (Mandatory Conformance Evidence):** Realized through the local transactional outbox pattern governed by `ADR-L5-0007` and `L5-B-02-003`. The host domain transaction writes its business state mutation and the `AuditObservationRequest` outbox record within a single database transaction. This guarantees zero dual-write inconsistency.
   - **Path B (Advisory Evidence):** Staged in an in-memory bounded ring buffer with an asynchronous background worker draining records to the `AuditIngestionPipeline`.
2. **Deduplication Boundary:** The `AuditIngestionPipeline` checks the unique composite key `(tenant_id, source_component, observation_id)` against a time-bounded deduplication index to prevent duplicate evidence generation (`L4-A-022-BH-007`).
3. **Tamper-Evident Storage Delegation:** The persistence realization for long-term immutable storage, cryptographic sealing, and historical archiving is specialized by `L5-D-04-003` under Dhruv's database/storage ownership.

### 5.6 Security, Trust & Isolation

- **Logical Trust Boundaries:** In accordance with `ADR-L5-0001`, all evidence crossing component or service boundaries must undergo admission filtering. Co-location of services within the same runtime does not bypass context enrichment or admission checks.
- **Tenant Scope Enforcement:** In accordance with `ADR-L5-0004`, evidence is partitioned by `tenant_id` and `institution_id`. Cross-tenant query execution is forbidden at the retrieval gateway.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-001` | Evidence Ingestion Path Separation | Implement a Dual-Path architecture: Synchronous Outbox for Mandatory Conformance Evidence, and Bounded Asynchronous Queue for Advisory Evidence. | `L4-A-022-BH-001`, `ADR-L5-0007` | Conformance-critical evidence must never be lost due to buffer drops, while advisory telemetry must never degrade transaction throughput or exhaust database connections. | `ADR-L5-0007` |
| `AD-002` | Evidence Aggregate Immutability | Evidence records are strictly append-only. Modification, replacement, or deletion of existing evidence records is prohibited at the architecture level. | `L4-A-022-BH-007`, `L4-A-022-BH-010` | Historical reconstruction and legal integrity require that evidence state cannot be mutated once generated. | None (Core Rule) — ADR necessity evaluated: this is a direct behavioral obligation from `L4-A-022-BH-007`; immutability is a constraint, not a technology choice. No architectural alternative exists at L5; ADR not required. |
| `AD-003` | Fail-Closed Enforcement for Mandatory Evidence | When mandatory evidence capture fails (e.g., outbox disk full, context unresolved), the host transaction must fail and roll back. | `L4-A-022-BH-001`, `C-001` | System state must never transition to a completed business status if the legally required compliance evidence cannot be recorded. | None (Core Rule) — ADR necessity evaluated: this behavior is derived directly from the `ADR-L5-0007` fail-closed invariant and `L4-A-022-BH-001` completion precondition. No independent cross-family technology choice is made; ADR not required. |
| `AD-004` | Baseline and Storage Separation | Decouple the capture and context baseline (`L5-D-04-001`) from the physical tamper-evident storage and export realization (`L5-D-04-003`). | Corpus Register, Workload Allocation | Preserves architectural separation between cross-cutting capture interceptors and specialized storage/cryptographic indexing engines owned by persistence architecture. | ADR required if storage engine selected |
| `AD-005` | Ingestion-Time Credential Redaction | Mandatory automated sanitization and redaction of secrets, private keys, authorization tokens, and credentials prior to evidence persistence. | `ADR-L5-0001`, `L5-D-03-001` | Prevents audit logs from becoming high-risk privilege escalation targets or violating data protection standards. | `ADR-L5-0001` |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| **Tenant Isolation** | Audit records strictly maintain Tenant and Institution metadata; queries require scoped admission. | `ADR-L5-0004` | `L5-A-04-001` |
| **Least Privilege** | Evidence retrieval gateway enforces read-only access restricted by classification policy. | `L4-A-023-BH-001` | `L5-D-02-001` |
| **Trust Integrity** | Non-transitive trust enforcement at ingestion and retrieval boundaries. | `ADR-L5-0001` | `L5-D-03-001` |
| **Credential Protection** | Zero machine credentials or private keys persisted in audit streams. | `AD-005` | `L5-D-03-004` |
| **Execution Stability** | Advisory buffer saturation drops oldest telemetry without blocking running execution threads. | `AD-001` | `L5-E-01-001` |
| **Audit Deduplication** | Idempotency guard prevents duplicate evidence records from duplicate event delivery. | `L4-A-022-BH-007` | `L5-D-04-001` |
| **Portability** | Architecture avoids hardcoded storage engine APIs; integrates via abstract persistence handoff. | Repository Standard | `L5-B-02-003` |

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| `CR-001` | Reusable evidence capture interceptors shall be implemented at defined runtime execution boundaries. | Section 5.1 | `L4-A-022-BH-001` |
| `CR-002` | Evidence generation shall distinguish between mandatory conformance evidence and advisory telemetry. | `AD-001` | `L4-A-022-BH-001` |
| `CR-003` | Captured evidence shall explicitly bind Tenant, Institution, Session, and Actor context. | Section 5.1 | `L4-A-022-BH-002`, `ADR-L5-0004` |
| `CR-004` | Missing or unresolvable Tenant context shall cause mandatory evidence capture to fail closed. | Section 5.3 | `ADR-L5-0004`, `C-004` |
| `CR-005` | Advisory evidence buffering shall utilize bounded memory/storage structures with non-blocking drop semantics on saturation. | `AD-001` | Section 5.3, `L5-E-01` |
| `CR-006` | Conformance-critical evidence shall commit atomically with host business state changes under `ADR-L5-0007`. | `AD-001`, `AD-003` | `ADR-L5-0007`, `L4-A-022-BH-007` |
| `CR-007` | The system shall emit `Audit Evidence Generated` via event broker only after evidence persistence is guaranteed. | Section 5.1 | `L4-A-022-BH-008`, `ADR-L5-0007` |
| `CR-008` | Evidence retrieval shall be exposed through a governed read-only gateway enforcing identity and scope authorization. | Section 5.1 | `L4-A-023-BH-001`, `L4-A-023-BH-007` |
| `CR-009` | The audit baseline shall not perform in-place mutation or updates to previously generated evidence records. | `AD-002` | `L4-A-022-BH-007`, `C-002` |
| `CR-010` | Ingestion boundaries shall automatically scrub or reject authentication credentials, private keys, and secrets. | `AD-005` | `L5-D-03-001`, `ADR-L5-0001` |
| `CR-011` | The audit capability shall not alter business lifecycle states, create competing business events, or approve transactions. | Section 2.3 | `HF-03`, `HF-04` |
| `CR-012` | Evidence deduplication shall verify composite keys `(tenant_id, source_component, observation_id)`. | Section 5.5 | `L4-A-022-BH-007` |
| `CR-013` | Long-term cryptographic sealing and tamper-evident storage realization shall be delegated to `L5-D-04-003`. | `AD-004` | Corpus Register |
| `CR-014` | The solution architecture shall not mandate specific database or message broker products without governed ADR approval. | `AD-004` | `L5_Technology_Selection_and_ADR_Methodology.md` |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| `OD-001` | Exact L4 BH Applicability | Upstream Traceability | None — Resolved against canonical `L4-A-022` and `L4-A-023`. | Lead Architect | **No (Closed)** | Mapped into Section 3.1 and Section 10. |
| `OD-002` | L4 Upstream Status | Upstream Authority | `L4-A-022`, `L4-A-023` carry Status: Draft in the repository. Treated as Locked under governance program assumption. Cannot be permanently closed until L4 governance approves or Lead Architect issues explicit authorization. | Lead Architect | **No (Conditionally Closed — pending L4 governance approval or explicit Lead Architect authorization referencing this specification)** | Update §3.1 status and this row once L4 governance is resolved. |
| `OD-003` | Tamper-Evident Storage & Export Technology | Technology / ADR | Specialized storage and cryptographic engine selection deferred to `L5-D-04-003`. | Dhruv / ADR Governance | No | Dhruv to author `L5-D-04-003` under Stage 3 allocation. |
| `OD-004` | Advisory Buffer Capacity Thresholds | Operational / Sizing | Specific buffer queue size and drop alert thresholds dependent on deployment scale. | Runtime / Operations (`L5-E-02`) | No | To be calibrated during deployment architecture phase without changing baseline. |
| `OD-005` | Deduplication Index Retention Horizon | Operational / Sizing | The time-bounded deduplication index (Section 5.5, composite key `(tenant_id, source_component, observation_id)`) requires a governed retention window. This is distinct from the advisory buffer capacity in OD-004 and from long-term evidence retention governed by `L5-D-04-003`. | Runtime / Operations (`L5-E-02`) | No | To be resolved during deployment architecture phase. Retention window must not be shorter than the maximum expected event redelivery window. |

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-022-BH-001` — Audit eligibility qualification and completed-fact precondition (`AD-001`, `CR-001`).
- `L4-A-022-BH-002` — Authoritative context resolution (Tenant, Institution, Session, Channel) (`CR-003`).
- `L4-A-022-BH-006` — Governed audit classification application without policy ownership transfer (`CR-002`).
- `L4-A-022-BH-007` — Immutable Audit Evidence creation and deduplication enforcement (`AD-002`, `CR-012`).
- `L4-A-022-BH-008` — Publication handoff for `Audit Evidence Generated` event (`CR-007`).
- `L4-A-022-BH-009` — Retrieval availability for authorized investigation (`CR-008`). Timeline Projection read-model availability and materialization (the second obligation of this BH) are owned by `L5-D-04-003`, which is responsible for projection storage, materialization queries, and the read-model boundary for `L4-A-023-BH-003`.
- `L4-A-023-BH-001` — Investigation access and authorization admission gate (`CR-008`).
- `L4-A-023-BH-002` — Locating canonical evidence by reference within authorized scope (`CR-008`).
- `L4-A-023-BH-007` — Providing read-only Investigation View without altering domain facts (`CR-008`).

### Unclaimed D04-Family BH Disposition

The following BHs from `L4-A-023` and `L4-A-024` are not realized by `L5-D-04-001` or `L5-D-04-002`. Ownership and realization are assigned as documented:

| BH ID | Assigned Realization Owner | Rationale |
|---|---|---|
| `L4-A-023-BH-003` | `L5-D-04-003` | Timeline Projection read-model retrieval and generation is a storage/query materialization responsibility belonging to Dhruv's Stage-3 tamper-evident storage and export specification. |
| `L4-A-023-BH-006` | `L5-D-04-003` | Publication of `Evidence Package Created` and `Timeline Projection Read Model Generated` events occurs after package assembly, which is owned by the retrieval and export layer in `L5-D-04-003`. |
| `L4-A-024-BH-001` | `L5-D-04-003` | Verification scope resolution (evidence, Tenant/classification boundary, Integrity Policy) requires access to the physical storage layer owned by `L5-D-04-003`. |
| `L4-A-024-BH-003` | `L5-D-04-003` | Verification of entity, workflow, and policy references within authorized scope is a storage-query responsibility owned by `L5-D-04-003`. |
| `L4-A-024-BH-006` | `L5-D-04-003` | Creation of one immutable Integrity Verification Result requires write capability to the tamper-evident store owned by `L5-D-04-003`. |
| `L4-A-024-BH-007` | `L5-D-04-003` | Publication of `Integrity Verification Result Recorded` and `Integrity Verification Failed` events occurs from the verification result creation boundary owned by `L5-D-04-003`. |
| `L4-A-024-BH-008` | `L5-D-04-003` | Making the immutable Integrity Verification Result available for authorized investigation requires the storage and retrieval capability owned by `L5-D-04-003`. |
| `L4-A-024-BH-009` | `L5-D-04-003` | Preservation of verification evidence including policy decisions, checks performed, and result identity is a tamper-evident persistence responsibility owned by `L5-D-04-003`. |

### Material Higher-Level & ADR Constraints

- `ADR-L5-0001` — Explicit trust boundaries; credential redaction at capture boundaries (`AD-005`, `CR-010`).
- `ADR-L5-0004` — Tenant and Institution isolation enforcement; strict partition boundary (`CR-004`).
- `ADR-L5-0007` — Atomic state-to-publication intent handoff; transactional outbox pattern (`AD-001`, `CR-006`).

### Core L5 Dependencies

- `L5-D-04-002` — Provenance graph, actor attribution model, and evidence data contracts consumed.
- `L5-D-04-003` — Cryptographic sealing and tamper-evident persistence handoff.
- `L5-B-02-003` — Transactional outbox persistence integration.
- `L5-D-01-001` / `L5-D-02-001` — Identity context and authorization enforcement.

### Architecture Decisions

- `AD-001` (Dual-Path Evidence Ingestion)
- `AD-002` (Immutable Evidence Aggregate Model)
- `AD-003` (Fail-Closed Enforcement for Mandatory Evidence)
- `AD-004` (Baseline and Storage Separation)
- `AD-005` (Ingestion-Time Credential Redaction)

### Conformance Requirements

- `CR-001` through `CR-014`