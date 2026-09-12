# L5-D-04-003 — Tamper-Evident Audit Storage, Access & Export Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-04-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-04 — Audit & Evidence Architecture  
**Canonical Technical Responsibility:** Integrity-protected audit storage, governed access and audit-specific export evidence/proof semantics without becoming owner of business records.  
**Technical Architecture Owner:** L5-D-04-003 — Tamper-Evident Audit Storage, Access & Export Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-12`  
**Depends On:** `L5-D-04-001`; `L5-D-04-002`; `L5-B-02-003`; `L5-B-02-004`; `L5-D-03-001`; `L5-D-03-002`; `L5-D-03-003`; `L5-D-02`; `L5-A-04`; `L5-B-05-002`; `L5-B-05-003`; applicable `L4-A-022`–`L4-A-027`.  
**Collaborating / Downstream Boundaries:** `L5-C-03-008`; `L5-E-03`; `L5-E-05`; `L5-A-08`.  
**Applicable ADRs:** `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0004`; `ADR-L5-0005`; `ADR-L5-0006`; `ADR-L5-0008`; D04-003 storage-and-integrity-topology ADR — **required but not yet allocated/approved**.

**Canonical Production Path:**  
`architecture/L5_Solution_Architecture/specifications/L5-D/L5-D-04-003_Tamper-Evident_Audit_Storage_Access_and_Export_Architecture.md`

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the audit-specific technical architecture that makes canonical Audit Evidence durably integrity-protected, independently verifiable, safely retrievable, preservable through archival and recovery, and exportable with verifiable provenance.

It consumes admitted evidence from `L5-D-04-001` and the canonical evidence/provenance model from `L5-D-04-002`.

It defines D04-003-specific storage semantics and integrity topology while consuming B02-owned physical persistence, transaction, concurrency and schema-evolution mechanics.

### 2.2 In Scope

- Audit-specific canonical evidence repository boundary.
- Append-only evidence persistence semantics.
- Deterministic integrity-input representation.
- Sequential integrity linkage.
- Sealed integrity checkpoints.
- Independently protected checkpoint anchoring.
- Safe concurrent stream-head advancement.
- Store-level evidence persistence idempotency.
- Integrity-verifiable evidence retrieval.
- Rebuildable non-authoritative search/query projections.
- Projection freshness/watermark architecture.
- Non-destructive audit-evidence archival.
- Governed restored read access.
- Audit-specific export evidence/proof assembly.
- Historical proof and persistence-evolution compatibility.
- Backup/recovery integrity reopening gates.
- Operational integrity signals.

### 2.3 Out of Scope / Non-Goals

This specification does not own:

- audit capture, admission or ingestion deduplication — `L5-D-04-001`;
- actor attribution, correlation, causality or evidence-envelope semantics — `L5-D-04-002`;
- database schema, SQL, persistence engine, transaction primitive or physical topology — `L5-B-02`;
- retention periods, legal-hold authority or disposition policy — upstream authority / `L5-B-05`;
- authorization policy, permissions or privileged-role definitions — `L5-D-02`;
- cryptographic algorithms or protection operations — `L5-D-03-002`;
- cryptographic key custody/lifecycle — `L5-D-03-003`;
- reusable bulk/file packaging, transfer, large-file delivery or generic delivery integrity — `L5-C-03-008`;
- backup/restore/DR implementation — `L5-E-03`;
- telemetry platform/alerting implementation — `L5-E-05`;
- job scheduling/cadence execution — `L5-A-08`;
- L6 investigation/export user experience.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source | Version / Status | Exact Reference | Technical Obligation for D04-003 |
|---|---|---|---|
| `L4-A-022` | v1.0.0 / Draft | BH-007 | Persist one immutable canonical Audit Evidence outcome and prevent duplicate authoritative evidence creation. |
| `L4-A-022` | v1.0.0 / Draft | BH-009 | Make canonical evidence available for authorized investigation and derived timeline/query use. |
| `L4-A-022` | v1.0.0 / Draft | BH-010 | Preserve evidence identity, source/context, actor, changes, correlation, causation, classification and policy references. |
| `L4-A-023` | v1.0.0 / Draft | BH-001–002 | Enforce authorized/scoped evidence discovery and retrieval. |
| `L4-A-023` | v1.0.0 / Draft | BH-005 | Support an immutable Evidence Package referencing canonical evidence. |
| `L4-A-023` | v1.0.0 / Draft | BH-007–008 | Preserve read-only investigation and access evidence. |
| `L4-A-024` | v1.0.0 / Draft | BH-002 | Verify evidence identity, immutability and historical execution context. |
| `L4-A-024` | v1.0.0 / Draft | BH-004–005 | Verify historical continuity; report missing/broken/duplicate integrity without repairing history. |
| `L4-A-024` | v1.0.0 / Draft | BH-006, BH-008–009 | Preserve immutable integrity-verification outcomes/evidence. |
| `L4-A-025` | v1.0.0 / Draft | BH-002–005, BH-007 | Retention/hold decisions govern disposition while evidence content remains unchanged. |
| `L4-A-026` | v1.0.0 / Draft | BH-001–008 | Archival changes governed Storage/Availability dimensions without mutating evidence/provenance. |
| `L4-A-027` | v1.0.0 / Draft | BH-001–005 | Governed restored access is temporary/read-only while evidence remains archived. |
| `L4-A-027` | v1.0.0 / Draft | BH-006–007 | Restoration closure duration/authority/event shall not be invented. |
| `L5-D-04-001` | v1.1.1 / Locked | §§4–8 | D04-001 owns capture/admission/deduplication and hands validated evidence to D04-003; publication occurs only after the required persistence milestone. |
| `L5-D-04-002` | v1.1.0 / Locked | §§2–5 | Supplies canonical `AuditEvidenceEnvelope` and provenance relationships consumed by D04-003. |
| `L5-B-02-003` | v1.0.1 / Draft | §§5–8 | Owner-local transaction and optimistic-concurrency mechanics govern D04-003 persistence commits. |
| `L5-B-02-004` | v1.0.1 / Draft | §§3–5 | Persistence evolution uses compatibility-preserving migration without rewriting authoritative historical meaning. |
| `L5-B-05-003` | v1.0.1 / Locked | §§3–8 | Retention/hold architecture authorizes technical disposition but does not define D04 physical integrity mechanics. |
| `ADR-L5-0002` | Architecture Approved | Decision | D03-002 owns cryptographic operation; D03-003 owns key custody/lifecycle. |
| `ADR-L5-0004` | Architecture Approved | Decision | Tenant/Institution scope isolation is mandatory. |
| `ADR-L5-0005/0006` | Architecture Approved | Decisions | Persistence ownership and owner-local transaction/concurrency boundaries remain B02-governed. |
| `ADR-L5-0008` | Architecture Approved | Decision | Persistence/proof evolution must be compatibility-safe rather than historical rewrite. |
| L5 Corpus `L5-C-03-008` | Frozen canonical responsibility | Corpus row | Owns reusable file/bulk exchange packaging, transfer and delivery-integrity mechanics. |

### 3.2 Authoritative Constraints

| ID | Constraint | Effect |
|---|---|---|
| C-001 | Canonical Audit Evidence is immutable after successful creation. | Canonical evidence payload is never updated in place. |
| C-002 | Query/index representations are not canonical evidence. | They are rebuildable from canonical evidence. |
| C-003 | Integrity findings do not authorize historical repair. | Verification is read-only. |
| C-004 | Tenant/Institution boundaries are preserved. | Integrity streams, projections and access may not silently widen scope. |
| C-005 | Evidence archival is non-destructive. | Archival changes control dimensions, not original evidence content. |
| C-006 | D04 does not own cryptographic key material. | Keys remain D03-owned. |
| C-007 | D04-003 does not own generic persistence mechanics. | B02 supplies transactions/concurrency/schema evolution. |
| C-008 | D04-003 does not own reusable file-transfer infrastructure. | C03-008 owns packaging/transfer/delivery mechanics. |

### 3.3 Assumptions

No material unsupported implementation assumption is treated as architecture fact.

Product choice, algorithm choice, checkpoint cadence and deployment-specific capacity values remain unresolved or delegated as recorded in Section 9.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 Owns

D04-003 owns:

- the audit-specific authoritative evidence repository contract;
- canonical evidence append semantics;
- integrity commitment representation requirements;
- evidence integrity stream semantics;
- sequential linkage semantics;
- checkpoint/seal semantics;
- checkpoint-anchor requirements;
- audit-specific integrity verification;
- audit-query projection authority/freshness semantics;
- audit archival integrity requirements;
- archived-evidence access integrity boundary;
- audit-specific export evidence/proof semantics;
- recovery-integrity reopening requirements.

### 4.2 Consumes / Collaborates With

| Specification | Consumed Capability | Boundary |
|---|---|---|
| `L5-D-04-001` | Capture/admission/ingestion deduplication | Supplies validated evidence; consumes persistence receipt. |
| `L5-D-04-002` | Canonical evidence/provenance model | D04-003 protects but does not redefine the envelope. |
| `L5-B-02-003` | Transaction/concurrency mechanics | Realizes atomic append/link/head advancement. |
| `L5-B-02-004` | Schema/data migration | Realizes compatibility-safe persistence/proof evolution. |
| `L5-D-03-002` | Cryptographic protection | Calculates/verifies cryptographic commitments and seals. |
| `L5-D-03-003` | Key management | Supplies governed key capabilities/references. |
| `L5-D-02` | Authorization | Supplies access/export/restoration decisions. |
| `L5-A-04` | Tenant/Institution isolation | Supplies validated scope. |
| `L5-B-05-002/003` | Disclosure and disposition controls | Supplies privacy/retention constraints. |
| `L5-C-03-008` | File exchange | Owns reusable archive/file packaging, transport and delivery integrity. |
| `L5-E-03` | Recovery | Owns backup/restore/DR execution. |
| `L5-E-05` | Observability | Owns operational telemetry/alerting mechanics. |
| `L5-A-08` | Job execution | May execute scheduled sealing/rebuild/archival work when dependency-ready. |

### 4.3 Must Not Own

D04-003 shall not own:

- business record authority;
- business event meaning/publisher authority;
- retention/legal-hold policy;
- authorization rules;
- cryptographic algorithm or key custody;
- database technology/schema;
- generic file-transfer packaging/delivery;
- backup/DR implementation;
- operational alert thresholds;
- UI/UX.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | State / Output | Failure Boundary |
|---|---|---|---|---|
| `AuditEvidencePersistenceBoundary` | Control Point | Accept validated canonical evidence for persistence. | Persistence attempt. | Invalid/incomplete context is rejected. |
| `EvidenceCommitmentCanonicalizer` | Component | Produce deterministic version-bound integrity input from immutable D04-002 evidence. | Integrity input + representation-version reference. | Unsupported/ambiguous representation fails closed. |
| `IntegrityStreamResolver` | Control Point | Resolve evidence to an authorized Tenant-bounded integrity stream. | Stream reference. | Invalid scope fails closed. |
| `SequentialIntegrityLinker` | Component | Link new evidence commitment to the committed predecessor. | Link metadata. | Concurrent head conflict cannot create silent fork. |
| `CanonicalEvidenceRepository` | Logical Repository | Represent canonical immutable evidence and D04 integrity metadata. | Authoritative evidence. | Failed commit is not reported as persisted. |
| `IntegrityStreamHead` | Technical State | Identify currently committed end of an integrity stream. | Current head reference/version. | Update only through governed B02 transaction. |
| `CheckpointSealCoordinator` | Component | Seal bounded committed stream ranges. | Sealed checkpoint. | Failed seal remains incomplete. |
| `CheckpointAnchorBoundary` | Control Point | Preserve independently protected reference to a closed checkpoint. | Anchor reference. | Anchor failure prevents strong closed-range claim. |
| `EvidenceIntegrityVerifier` | Component | Verify content commitment, chain continuity, checkpoint/anchor evidence. | Verification result/findings. | Never repairs evidence. |
| `EvidenceProjectionBuilder` | Worker/Component | Build searchable non-authoritative evidence projection. | Query projection + watermark. | Failure does not mutate canonical evidence. |
| `EvidenceAccessGateway` | Control Point | Authorize discovery, resolve canonical evidence and return read-only results. | Canonical evidence/proofs. | Fails closed on scope/auth/integrity requirement failure. |
| `AuditArchiveCoordinator` | Component | Realize authorized evidence archival while preserving evidence/proofs. | Archival execution result. | Partial archival cannot claim completion. |
| `ArchivedEvidenceAccessController` | Control Point | Realize authorized read-only restored access. | Scoped access. | Does not invent closure policy. |
| `EvidenceExportProofAssembler` | Component | Construct audit-specific evidence selection/proof manifest for export. | Evidence/proof handoff to C03-008. | No successful export handoff when integrity assembly fails. |
| `RecoveryIntegrityGate` | Control Point | Verify recovered evidence continuity before canonical writes reopen. | Reopen/hold outcome. | Unverified recovery cannot silently resume append. |
| `OperationalIntegritySignalBoundary` | Control Point | Surface D04 integrity conditions to E05. | Technical signals. | Does not define alert product/threshold. |

### 5.2 Deterministic Evidence Commitment

Cryptographic integrity shall not depend on incidental serializer behavior.

Before cryptographic commitment:

```text
D04-002 AuditEvidenceEnvelope
        ↓
EvidenceCommitmentCanonicalizer
        ↓
deterministic integrity representation
        +
integrity-representation version reference
        ↓
D03-002 cryptographic operation
        ↓
evidence content commitment
```

The canonicalizer shall include only immutable evidence information required for integrity protection.

The integrity-representation version is technical metadata and shall not alter the business/evidence meaning of the D04-002 envelope.

No JSON, Protobuf, Avro, hash algorithm or other concrete representation is selected here.

### 5.3 Integrity Stream Boundary

Every canonical evidence append resolves to one authorized integrity stream.

Minimum invariant:

```text
one integrity stream
must never span different Tenant authority boundaries
```

Within a Tenant, exact stream partitioning is governed by the pending D04-003 ADR.

A stream partitioning strategy shall:

- be deterministic for new appends;
- preserve unambiguous predecessor resolution;
- permit historical continuity verification;
- avoid using query projection state as integrity authority;
- prevent an implementation from silently changing stream assignment in a way that breaks prior proofs.

### 5.4 Atomic Canonical Append

The authoritative append path is:

```text
validated AuditEvidenceEnvelope
        ↓
deterministic evidence commitment
        ↓
resolve integrity stream
        ↓
read current stream head + concurrency precondition
        ↓
construct predecessor linkage
        ↓
B02 OWNER-LOCAL ATOMIC TRANSACTION
        ├── persist canonical evidence
        ├── persist integrity-link metadata
        └── compare-and-advance IntegrityStreamHead
        ↓
COMMIT
        ↓
EvidencePersistenceReceipt
```

Canonical evidence, predecessor linkage and stream-head advancement form one logical commit boundary.

A stream-head conflict shall fail the attempted commit rather than create an unnoticed fork.

Retry/re-evaluation shall use the newly committed head according to B02-owned concurrency mechanics.

No distributed transaction across D03, C02 or other owners is introduced.

### 5.5 Store-Level Idempotency

D04-001 remains owner of audit-ingestion deduplication.

D04-003 additionally guarantees safe persistence retry after uncertain execution outcome.

For a canonical evidence identity:

```text
not previously persisted
    → perform canonical append

previously persisted
+ same canonical evidence commitment
    → return existing persistence receipt
    → do not append a duplicate

previously persisted
+ different canonical evidence commitment
    → integrity conflict
    → fail closed
```

A conflicting second representation of the same canonical evidence identity shall never silently replace the first.

### 5.6 Persistence Milestone and D04-001 Handoff

The persistence milestone required by D04-001 is:

```text
canonical evidence committed
+
sequential integrity linkage committed
+
IntegrityStreamHead committed
```

At that point D04-003 returns:

```text
EvidencePersistenceReceipt {
    canonical evidence reference
    integrity stream/position reference
    committed integrity reference
}
```

This receipt is conceptual, not a wire schema.

D04-001 may then perform its canonical `Audit Evidence Generated` publication handoff according to its locked architecture.

Periodic checkpoint sealing is a subsequent integrity-strengthening control and is **not** required to make every individually committed evidence record canonical.

This separation prevents checkpoint cadence from becoming an implicit business-event publication delay.

### 5.7 Sequential Integrity Linkage

Each committed evidence record participates in ordered integrity continuity:

```text
prior committed integrity value
        +
current evidence commitment
        +
required stream position/context
        ↓
new integrity value
```

The sequence protects detection of:

- evidence-content modification;
- deletion from inside an otherwise retained sequence;
- reordering;
- predecessor substitution;
- undetected append-fork attempts when combined with head concurrency control.

The exact cryptographic construction remains D03/ADR-governed.

### 5.8 Checkpoint Sealing and Independent Anchoring

Sequential integrity linkage alone does not establish the same rollback resistance as an independently protected closed checkpoint.

D04-003 therefore uses:

```text
committed integrity stream range
        ↓
CheckpointSealCoordinator
        ↓
D03-002 cryptographic protection
        ↓
SealedCheckpoint
        ↓
CheckpointAnchorBoundary
        ↓
independently protected checkpoint anchor reference
```

A sealed checkpoint establishes a verifiable closed range.

The anchor boundary shall be logically protected from ordinary evidence write authority such that deleting or rolling back evidence and its colocated checkpoint cannot silently recreate a valid anchored state.

This specification does not require:

- blockchain;
- a separate physical server;
- a specific HSM;
- a specific object store;
- a cloud service;
- a particular algorithm.

The conforming anchor realization belongs in the pending D04-003 ADR.

Evidence appended after the latest anchored checkpoint forms an **open tail**. The architecture shall not claim the same rollback-resistance property for the open tail as for an anchored closed range.

Checkpoint frequency and acceptable open-tail exposure are governed deployment/security parameters recorded in Section 9.

### 5.9 Tamper-Detection Coverage

The architecture provides the following logical detection coverage:

| Condition | Detection Mechanism |
|---|---|
| Evidence content modification | Canonical evidence commitment mismatch. |
| Middle-record removal | Sequential continuity failure. |
| Reordering | Sequential predecessor/position verification failure. |
| Concurrent fork attempt | B02 stream-head concurrency conflict. |
| Closed-range rollback | Checkpoint/independent-anchor disagreement. |
| Query projection corruption | Canonical re-resolution and verification. |
| Export alteration | Export evidence/proof verification failure. |
| Recovery discontinuity | RecoveryIntegrityGate continuity verification failure. |

Integrity verification reports findings. It shall not rewrite evidence to force a valid result.

### 5.10 Canonical Evidence vs Query Projection

Two authority classes exist:

```text
CanonicalEvidenceRepository
        =
authoritative Audit Evidence

EvidenceQueryProjection
        =
derived / rebuildable / non-authoritative
```

The query projection may contain searchable references and approved derived attributes required for evidence discovery.

It shall not become an alternate canonical evidence repository.

### 5.11 Projection Watermark and Reconciliation

Each projection maintains a technical watermark identifying the last canonical integrity position successfully incorporated.

```text
canonical committed position
        ↓
projection processing
        ↓
ProjectionWatermark
```

For broad search/investigation:

```text
projection watermark
        compared with
canonical committed position
```

If the projection is known to lag:

- the search result shall not be represented as complete beyond its watermark;
- exact canonical evidence retrieval remains possible where the canonical reference is known and authorization allows it;
- rebuild/reconciliation may occur from canonical evidence;
- canonical evidence shall not be modified to match a projection.

Projection rebuild shall converge from canonical evidence/integrity positions rather than treating existing projection contents as authority.

### 5.12 Evidence Access Flow

```text
investigation / verification request
        ↓
D02 authorization decision
        +
A04 Tenant/Institution scope
        ↓
EvidenceAccessGateway
        ↓
query projection for discovery where appropriate
        ↓
canonical evidence references
        ↓
CanonicalEvidenceRepository
        ↓
EvidenceIntegrityVerifier
        ↓
authorized read-only result
```

Authorization to query evidence does not grant authority to mutate evidence, stream heads, checkpoints or anchors.

### 5.13 Immutable Evidence vs Storage / Availability Control

Canonical evidence content and its integrity commitment are separate from the upstream-defined Storage and Availability lifecycle dimensions.

```text
Immutable Audit Evidence
        ≠
Storage / Availability control representation
```

Therefore transitions such as:

```text
Active Storage → Archived Storage
Available → Restricted Access
Restricted Access → Restored Access
```

shall not rewrite the canonical `AuditEvidenceEnvelope` or invalidate its original content commitment.

D04-003 may persist the technical representation of authorized Storage/Availability control outcomes, but the values and transition authority remain upstream-defined.

### 5.14 Governed Archival

```text
authoritative archival eligibility
        ↓
B05-003 disposition enforcement
        ↓
AuditArchiveCoordinator
        ↓
create/establish archived representation
        ↓
verify canonical evidence identity
        +
verify evidence integrity
        +
verify required proof metadata
        ↓
commit authorized Storage / Availability outcome
        +
required publication intent through canonical owner boundary
        ↓
only then:
archival execution considered complete
```

A physical copy/movement alone is not archival completion.

Likewise, a control-state transition without a valid archived evidence representation is not archival completion.

Partial execution shall preserve the last valid governed outcome and shall not publish/claim successful archival.

### 5.15 Archived Evidence Restored Access

```text
Archived Storage
+
Restricted Access
        ↓
authorized restoration context
        ↓
verify canonical evidence identity/integrity
        ↓
ArchivedEvidenceAccessController
        ↓
read-only Restored Access
```

The canonical evidence remains archived.

The architecture shall not invent:

- restoration duration;
- expiry time;
- closure actor;
- closure command;
- closure event.

Those remain upstream unresolved items.

### 5.16 Audit-Specific Export Architecture

D04-003 owns **audit-specific export selection/proof semantics**, not the reusable file-transfer platform.

```text
authorized audit evidence scope
        ↓
resolve canonical evidence
        ↓
verify canonical integrity
        ↓
apply approved disclosure/minimization decision where required
        ↓
EvidenceExportProofAssembler
        ↓
audit evidence/proof manifest
        +
permitted evidence representations
        ↓
L5-C-03-008
        ↓
generic packaging / transfer / delivery
```

The audit-specific export proof material shall bind delivered audit evidence to:

- canonical evidence identities;
- canonical integrity references;
- applicable checkpoint/anchor proof references;
- authorized evidence scope;
- applicable representation/protection version references.

D04-003 shall not define:

- archive file format;
- compression format;
- transport protocol;
- file chunking;
- generic large-transfer mechanics;
- generic delivery acknowledgement.

Those belong to `L5-C-03-008`.

An exported copy is never a new canonical Audit Evidence owner.

### 5.17 Persistence and Proof Evolution

Persistence/schema/proof evolution shall preserve historical verifiability.

Changes shall not:

- rewrite historical Audit Evidence merely to conform to a new physical representation;
- destroy the ability to resolve the original integrity-representation version;
- invalidate historical checkpoint/protection references without a governed continuity mechanism;
- silently replace old proof history as though the replacement proof had always existed.

Where a newer supplementary proof is required, it shall remain traceable to the preserved historical evidence/proof basis.

B02-004 owns the physical migration mechanics.

D04-003 owns the audit-specific requirement that historical integrity evidence remains verifiable through those migrations.

### 5.18 Recovery Integrity Gate

Backup/restore/DR execution belongs to `L5-E-03`.

Before recovered D04-003 storage is reopened for canonical append:

```text
recovered canonical evidence
+
recovered integrity metadata
+
recovered checkpoint records
+
recovered anchor references
        ↓
verify latest applicable anchored checkpoint
        ↓
verify continuity from checkpoint to recovered stream head
        ↓
verify recovered IntegrityStreamHead
        ↓
RecoveryIntegrityGate
     ↙             ↘
verified            unresolved / failed
   ↓                       ↓
reopen append        keep canonical append closed
```

An implementation shall not silently start a new integrity chain to disguise an unresolved recovery discontinuity.

Projection state may be rebuilt and does not block canonical recovery if canonical evidence/proof continuity has been established.

### 5.19 Logical Privilege Separation

The architecture requires logical separation between technical authorities for:

- canonical evidence append;
- checkpoint/seal initiation;
- canonical evidence query/read;
- audit export preparation;
- archive/restored-access execution;
- cryptographic operation/key custody.

At minimum:

```text
read/export authority
    ≠
canonical evidence mutation authority

D04 storage authority
    ≠
D03 key-custody authority
```

The implementation mechanism for this separation remains D02/D03/runtime-governed.

No specific human role vocabulary is introduced.

### 5.20 Operational Integrity Signals

D04-003 shall expose technical conditions sufficient for E05-owned observation of:

- canonical append failure;
- evidence-identity integrity conflict;
- stream-head concurrency conflict;
- sequential continuity failure;
- checkpoint-sealing failure;
- checkpoint backlog/open-tail position;
- checkpoint-anchor failure;
- projection watermark lag;
- projection reconciliation failure;
- integrity-verification failure;
- archive verification/incompletion;
- restored-access integrity failure;
- export-proof generation/verification failure;
- recovery continuity failure.

No threshold, timeout, retry count, alert product or SLA value is defined here.

---

## 6. Architecture & Technology Decisions

| ID | Decision | Selected Position | Basis | ADR |
|---|---|---|---|---|
| AD-001 | Canonical/query topology | One authoritative immutable evidence repository + rebuildable non-authoritative query projection. | L4-A-022/023; D04 boundary | Required D04-003 ADR |
| AD-002 | Integrity topology | Sequential integrity linkage + periodic sealed checkpoints. | L4-A-024; D04 responsibility | Required D04-003 ADR |
| AD-003 | Closed-range rollback protection | Sealed checkpoints require an independently protected anchor boundary. | Necessary technical derivation from tamper-evidence responsibility | Required D04-003 ADR |
| AD-004 | Append consistency | Canonical evidence + integrity link + stream-head advancement commit atomically through B02. | B02-003 / ADR-L5-0006 | Existing ADRs |
| AD-005 | Persistence retry | Same evidence identity + same commitment is idempotent; same identity + different commitment is integrity conflict. | D04 persistence correctness | Local |
| AD-006 | Commitment representation | Integrity input is deterministic and explicitly version-bound. | Historical verification necessity; D04-002 serializer neutrality | Local |
| AD-007 | Evidence/control separation | Immutable evidence content is separate from mutable authorized Storage/Availability control representation. | L4-A-026/027 | Local |
| AD-008 | Persistence milestone | Append/link/head commit is sufficient for D04-001 persistence receipt; checkpoint is subsequent strengthening. | Locked D04-001 CR-007 boundary | Local |
| AD-009 | Projection completeness | Query projection carries a canonical-position watermark and cannot claim completeness beyond it. | L4-A-023 investigation correctness | Local |
| AD-010 | Recovery reopening | Canonical append remains closed until recovered integrity continuity is verified. | D04 tamper-evident responsibility | Local |
| AD-011 | Export boundary | D04-003 owns audit-specific evidence/proof semantics; C03-008 owns generic packaging/transfer/delivery. | Frozen corpus responsibility | Local boundary decision |
| AD-012 | Privilege separation | Append, read/export and key-custody authorities remain logically separated. | ADR-L5-0001/0002 | Existing ADRs |

No database, search engine, object store, cryptographic algorithm, KMS/HSM, cloud provider, runtime or deployment product is selected.

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Architectural Impact | Owner / Collaborator |
|---|---|---|
| Tenant isolation | Integrity streams/checkpoints/queries never silently cross Tenant authority. | A04 / D04 |
| Key custody | Raw key material remains outside D04-003. | D03 |
| Authorization | Access/export/restoration consumes D02 decisions and fails closed when absent. | D02 |
| Read vs write authority | Query/export capability does not imply canonical mutation authority. | D02 / D04 |
| Projection degradation | Search may degrade while canonical evidence remains valid. | D04 / E05 |
| Tamper rollback resistance | Anchored closed ranges have stronger rollback protection than the open tail. | D04 / D03 |
| Historical portability | Representation/proof versions remain resolvable through migration. | D04 / B02 |
| Recoverability | Canonical append reopens only after integrity continuity verification. | D04 / E03 |
| Export | Audit proof semantics stay D04-owned; generic delivery stays C03-008-owned. | D04 / C03 |

---

## 8. Architecture Conformance Requirements

| ID | Requirement | Derived From |
|---|---|---|
| CR-001 | Canonical Audit Evidence shall not be updated in place after successful persistence. | C-001 |
| CR-002 | Integrity input shall be deterministic and retain its integrity-representation version reference. | AD-006 |
| CR-003 | Integrity streams shall not cross Tenant authority boundaries. | AD-001/002; ADR-L5-0004 |
| CR-004 | Canonical evidence, predecessor link and stream-head advancement shall commit atomically. | AD-004 |
| CR-005 | Concurrent stream-head change shall not create a silent fork. | AD-004 |
| CR-006 | Persistence retry of identical canonical evidence shall not create duplicate evidence. | AD-005 |
| CR-007 | Reuse of a canonical evidence identity with a different integrity commitment shall fail as an integrity conflict. | AD-005 |
| CR-008 | D04-003 shall return persistence success only after canonical evidence/link/head commit succeeds. | AD-008 |
| CR-009 | Periodic checkpoint sealing shall not be silently treated as a prerequisite for every `Audit Evidence Generated` publication. | AD-008 |
| CR-010 | A sealed checkpoint shall have an independently protected anchor before D04-003 claims closed-range rollback resistance. | AD-003 |
| CR-011 | D04-003 shall not claim identical rollback resistance for the unanchored open tail. | §5.8 |
| CR-012 | Integrity verification shall be read-only and shall surface rather than repair historical inconsistencies. | §5.9 |
| CR-013 | Query projections shall remain rebuildable and non-authoritative. | AD-001 |
| CR-014 | Query projections shall maintain a canonical-position watermark sufficient to identify known lag. | AD-009 |
| CR-015 | A known-lagging projection shall not be presented as complete beyond its watermark. | AD-009 |
| CR-016 | Canonical evidence shall be re-resolved from the authoritative repository before a projection result is treated as canonical evidence. | §5.10–5.12 |
| CR-017 | Archival shall not rewrite the original AuditEvidenceEnvelope or original evidence content commitment. | AD-007 |
| CR-018 | Archival completion shall not be claimed until archived representation integrity and required Storage/Availability outcomes have completed. | §5.14 |
| CR-019 | Restored archived access shall remain read-only and shall not change `Archived Storage`. | §5.15 |
| CR-020 | D04-003 shall not invent restoration duration, expiry, closure authority or closure event. | L4-A-027 |
| CR-021 | Audit-specific export proof material shall remain bound to canonical evidence identities and integrity references. | AD-011 |
| CR-022 | Generic export packaging/transfer/delivery mechanics shall remain C03-008-owned. | AD-011 |
| CR-023 | Persistence/proof migration shall preserve the ability to verify retained historical evidence. | §5.17 |
| CR-024 | Recovery shall not reopen canonical append until applicable recovered integrity continuity is verified. | AD-010 |
| CR-025 | Recovery shall not silently start a replacement integrity chain to hide unresolved discontinuity. | AD-010 |
| CR-026 | Read/export authority shall not automatically include canonical evidence mutation authority. | AD-012 |
| CR-027 | D04-003 shall not own raw cryptographic key material. | ADR-L5-0002 |
| CR-028 | Material D04 integrity failures shall be observable through E05-consumable technical signals. | §5.20 |

---

## 9. Open Decisions & Deferred Design

| ID | Item | Classification | Blocking? | Required Resolution |
|---|---|---|---|---|
| OD-001 | D04-003 storage/query/integrity/checkpoint-anchor topology ADR | ADR Required | **Yes for Architecture Approved** | Allocate and approve one Specification-scope ADR covering AD-001–AD-003 and related stream topology. |
| OD-002 | Material `L4-A-022`–`027` sources remain Draft | Upstream/Governance | **Yes for Architecture Approved** | Promote applicable L4 or explicitly authorize Stage-3 approval-level use. |
| OD-003 | Restoration expiry/closure basis | Upstream | No for core storage; blocks automatic closure semantics | Resolve upstream `L4-A-027` gap. |
| OD-004 | Checkpoint cadence/range and maximum acceptable open-tail exposure | Security / Operations parameter | No for architecture Draft; required before production deployment | Establish governed security/operations values; do not invent them here. |
| OD-005 | Intra-Tenant integrity-stream partitioning/scaling strategy | ADR realization | **Yes for final D04-003 ADR closure** | Pending ADR shall choose deterministic partitioning constraints compatible with integrity verification. |
| OD-006 | Concrete persistence/search/object-storage technology | Technology | No | Resolve through applicable B02/technology ADR governance if a product choice becomes material. |
| OD-007 | D04-002 states “export archive packaging” is D04-003-owned while frozen C03-008 corpus assigns reusable export packaging/transfer to C03-008 | Repository Boundary Clarification | No under current governance precedence | Treat D04-003 as owner of audit-specific evidence/proof export semantics and C03-008 as generic packaging/transfer owner; harmonize wording when D04-002 is next amended. |
| OD-008 | C03-008 concrete integration contract is not yet stable/available for consumption | Dependency | No for D04 audit-proof architecture | Finalize exact handoff when C03-008 becomes dependency-ready. |

Because `OD-001` and `OD-002` remain blocking, this specification remains `Draft`.

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-022-BH-007/009/010` — immutable canonical evidence and availability.
- `L4-A-023-BH-001/002/005/007/008` — authorized discovery/access/Evidence Package.
- `L4-A-024-BH-002/004/005/006/008/009` — integrity verification.
- `L4-A-025-BH-002–005/007` — retention/disposition authority.
- `L4-A-026-BH-001–008` — governed non-destructive archival.
- `L4-A-027-BH-001–007` — restored archived access and explicit closure gap.

### Primary L5 Dependencies

- `L5-D-04-001` — capture/admission/deduplication/publication boundary.
- `L5-D-04-002` — canonical evidence/provenance model.
- `L5-B-02-003/004` — persistence concurrency and migration.
- `L5-D-03-001/002/003` — security, cryptography and keys.
- `L5-D-02` — authorization.
- `L5-A-04` — Tenant/Institution isolation.
- `L5-B-05-002/003` — disclosure and retention/disposition controls.
- `L5-C-03-008` — generic packaging/transfer/delivery.
- `L5-E-03` — recovery.
- `L5-E-05` — operational observation.

### Architecture Decisions

`AD-001`–`AD-012`

### Conformance Requirements

`CR-001`–`CR-028`

### Blocking Items

- `OD-001` — D04-003 ADR required.
- `OD-002` — L4 approval/governance authority required.
- `OD-005` — exact intra-Tenant stream topology must close in the pending ADR.

**Current Status:** `Draft`
