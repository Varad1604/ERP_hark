# L5-D-04-003 — Tamper-Evident Audit Storage, Access & Export Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-04-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-04 — Audit & Evidence Architecture  
**Canonical Technical Responsibility:** Integrity-protected audit storage, governed access and audit-specific export evidence/proof semantics without becoming owner of business records.  
**Technical Architecture Owner:** L5-D-04-003 — Tamper-Evident Audit Storage, Access & Export Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.3`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-22`  
**Depends On:** `L5-D-04-001`; `L5-D-04-002`; `L5-B-02-003`; `L5-B-02-004`; `L5-D-03-001`; `L5-D-03-002`; `L5-D-03-003`; `L5-D-02`; `L5-A-04`; `L5-B-05-002`; `L5-B-05-003`; applicable `L4-A-022`–`L4-A-027`.  
**Collaborating / Downstream Boundaries:** `L5-C-03-008`; `L5-E-03`; `L5-E-05`; `L5-A-08`.  
**Applicable ADRs:** `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0004`; `ADR-L5-0005`; `ADR-L5-0006`; `ADR-L5-0008`; `ADR-L5-0012`.  
**Governance Authorization:** `L5_Dhruv_Stage3_D04-003_Architecture_Approval_Decision_2026-09-12.md`; `L5_Dhruv_Stage3_D04-003_Traceability_Refinement_Approval_2026-09-13.md`.

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
- Tenant-local versioned integrity-stream assignment.
- Sequential integrity linkage.
- Sealed integrity checkpoints.
- Independently protected checkpoint anchoring.
- Safe concurrent stream-head advancement.
- Store-level evidence persistence idempotency.
- Integrity-verifiable evidence retrieval.
- Durable preservation of immutable `Integrity Verification Result` outcomes.
- Immutable investigation `Evidence Package` reference-manifest semantics without duplicating canonical evidence.
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

For this controlled Stage-3 validation, the cited L4 BHs below are authorized behavioral authority by `L5_Dhruv_Stage3_D04-003_Architecture_Approval_Decision_2026-09-12.md` even though their own L4 metadata remains `Draft`. That authorization is limited to this specification and does not globally promote the L4 files.

| Source | Version / Status | Exact Reference | Technical Obligation for D04-003 |
|---|---|---|---|
| `L4-A-022` | v1.0.0 / Draft; Stage-3 authorized behavioral authority | BH-007 | Persist one immutable canonical Audit Evidence outcome and prevent duplicate authoritative evidence creation. |
| `L4-A-022` | same | BH-009 | Make canonical evidence available for authorized investigation and derived timeline/query use. |
| `L4-A-022` | same | BH-010 | Preserve evidence identity, source/context, actor, changes, correlation, causation, classification and policy references. |
| `L4-A-023` | v1.0.0 / Draft; Stage-3 authorized behavioral authority | BH-001–002 | Enforce authorized/scoped evidence discovery and retrieval. |
| `L4-A-023` | same | BH-005 | Support an immutable Evidence Package referencing canonical evidence. |
| `L4-A-023` | same | BH-007–008 | Preserve read-only investigation and access evidence. |
| `L4-A-024` | v1.0.0 / Draft; Stage-3 authorized behavioral authority | BH-002 | Verify evidence identity, immutability and historical execution context. |
| `L4-A-024` | same | BH-004–005 | Verify historical continuity; report missing/broken/duplicate integrity without repairing history. |
| `L4-A-024` | same | BH-006, BH-008–009 | Preserve immutable integrity-verification outcomes/evidence. |
| `L4-A-025` | v1.0.0 / Draft; Stage-3 authorized behavioral authority | BH-002–005, BH-007 | Retention/hold decisions govern disposition while evidence content remains unchanged. |
| `L4-A-026` | v1.0.0 / Draft; Stage-3 authorized behavioral authority | BH-001–008 | Archival changes governed Storage/Availability dimensions without mutating evidence/provenance. |
| `L4-A-027` | v1.0.0 / Draft; Stage-3 authorized behavioral authority | BH-001–005 | Governed restored access is temporary/read-only while evidence remains archived. |
| `L4-A-027` | same | BH-006–007 | Restoration closure duration/authority/event shall not be invented. |
| `L5-D-04-001` | v1.1.1 / Locked | §§4–8 | D04-001 owns capture/admission/deduplication and hands validated evidence to D04-003; publication occurs only after the required persistence milestone. |
| `L5-D-04-002` | v1.1.1 / Locked | §§2–5 | Supplies canonical `AuditEvidenceEnvelope` and provenance relationships consumed by D04-003; physical persistence remains B02-owned, cryptographic protection D03-owned, and reusable packaging/transfer C03-008-owned. |
| `L5-B-02-003` | v1.1.0 / Architecture Approved | §§5–8 | Owner-local transaction and optimistic-concurrency mechanics govern D04-003 persistence commits. |
| `L5-B-02-004` | v1.3.0 / Architecture Approved | §§3–5 | Persistence evolution uses compatibility-preserving migration without rewriting authoritative historical meaning. |
| `L5-B-05-003` | v1.0.1 / Locked | §§3–8 | Retention/hold architecture authorizes technical disposition but does not define D04 physical integrity mechanics. |
| `ADR-L5-0002` | Architecture Approved | Decision | D03-002 owns cryptographic operation; D03-003 owns key custody/lifecycle. |
| `ADR-L5-0004` | Architecture Approved | Decision | Tenant/Institution scope isolation is mandatory. |
| `ADR-L5-0005/0006` | Architecture Approved | Decisions | Persistence ownership and owner-local transaction/concurrency boundaries remain B02-governed. |
| `ADR-L5-0008` | Architecture Approved | Decision | Persistence/proof evolution must be compatibility-safe rather than historical rewrite. |
| `ADR-L5-0012` | Architecture Approved | Decision §§7–11 | Defines canonical/query topology, Tenant-local versioned integrity streams, sequential linkage, checkpoint sealing and independently protected anchoring. |
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
| C-009 | Integrity-stream topology changes may not rewrite historical stream assignment. | Topology evolves through versioned integrity epochs under `ADR-L5-0012`. |
| C-010 | Integrity Verification Result and Evidence Package records are immutable audit artifacts, not replacements for canonical Audit Evidence. | They reference authoritative evidence/proof scope and never mutate source evidence. |

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
- evidence integrity stream and integrity-epoch semantics;
- sequential linkage semantics;
- checkpoint/seal semantics;
- checkpoint-anchor requirements;
- audit-specific integrity verification;
- durable D04 persistence semantics for immutable `Integrity Verification Result` outcomes;
- immutable investigation `Evidence Package` reference-manifest semantics that point to canonical evidence without duplicating it;
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
| `L5-B-02-003` | Transaction/concurrency mechanics | Realizes atomic append/link/head advancement and durable audit-artifact persistence. |
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
| `IntegrityStreamResolver` | Control Point | Resolve evidence to an authorized Tenant-local integrity stream using the active assignment version/epoch. | Stream + epoch reference. | Invalid scope/assignment fails closed. |
| `SequentialIntegrityLinker` | Component | Link new evidence commitment to the committed predecessor. | Link metadata. | Concurrent head conflict cannot create silent fork. |
| `CanonicalEvidenceRepository` | Logical Repository | Represent canonical immutable evidence and D04 integrity metadata. | Authoritative evidence. | Failed commit is not reported as persisted. |
| `IntegrityStreamHead` | Technical State | Identify currently committed end of an integrity stream. | Current head reference/version. | Update only through governed B02 transaction. |
| `CheckpointSealCoordinator` | Component | Seal bounded committed stream ranges and integrity-epoch boundaries. | Sealed checkpoint. | Failed seal remains incomplete. |
| `CheckpointAnchorBoundary` | Control Point | Preserve independently protected reference to a closed checkpoint. | Anchor reference. | Anchor failure prevents strong closed-range claim. |
| `EvidenceIntegrityVerifier` | Component | Verify content commitment, chain continuity, checkpoint/anchor and epoch-transition evidence. | Verification outcome/findings. | Never repairs evidence. |
| `IntegrityVerificationResultRecorder` | Component | Convert a completed verification outcome into one immutable durable `Integrity Verification Result` referencing verified evidence/scope and proof context. | Durable immutable result reference. | Failed result persistence prevents a durable-completion claim for the verification outcome. |
| `EvidencePackageReferenceAssembler` | Component | Create an immutable investigation `Evidence Package` reference manifest over authorized canonical evidence and relevant proof/result references without duplicating canonical evidence. | Immutable Evidence Package reference manifest. | Invalid/missing references prevent package-completion claim. |
| `EvidenceProjectionBuilder` | Worker/Component | Build searchable non-authoritative evidence projection. | Query projection + watermark. | Failure does not mutate canonical evidence. |
| `EvidenceAccessGateway` | Control Point | Authorize discovery, resolve canonical evidence and return read-only results. | Canonical evidence/proofs. | Fails closed on scope/auth/integrity requirement failure. |
| `AuditArchiveCoordinator` | Component | Realize authorized evidence archival while preserving evidence/proofs. | Archival execution result. | Partial archival cannot claim completion. |
| `ArchivedEvidenceAccessController` | Control Point | Realize authorized read-only restored access. | Scoped access. | Does not invent closure policy. |
| `EvidenceExportProofAssembler` | Component | Construct audit-specific evidence selection/proof manifest for export. | Evidence/proof handoff to C03-008. | No successful export handoff when integrity assembly fails. |
| `RecoveryIntegrityGate` | Control Point | Verify recovered evidence continuity before canonical writes reopen. | Reopen/hold outcome. | Unverified recovery cannot silently resume append. |
| `OperationalIntegritySignalBoundary` | Control Point | Surface D04 integrity conditions to E05. | Technical signals. | Does not define alert product/threshold. |

### 5.2 Deterministic Evidence Commitment

Cryptographic integrity shall not depend on incidental serializer behavior.

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

### 5.3 Tenant-Local Versioned Integrity Streams

Every canonical evidence append resolves to one authorized integrity stream.

```text
one integrity stream
must never span different Tenant authority boundaries
```

Under `ADR-L5-0012`:

1. a Tenant may use one or multiple logical integrity streams;
2. evidence assignment uses a versioned deterministic assignment rule;
3. an assignment version defines a Tenant integrity epoch;
4. evidence already committed in an epoch never changes stream assignment due to later scaling/topology change;
5. a topology change becomes effective only after the prior epoch is closed by an anchored checkpoint boundary sufficient for historical verification;
6. the successor epoch retains the predecessor anchored-epoch reference needed to verify continuity;
7. historical evidence shall not be rewritten or resharded in place merely to rebalance integrity streams;
8. the exact product-level routing/hash implementation remains downstream design.

The stream-assignment strategy shall preserve unambiguous predecessor resolution, historical continuity and independence from non-authoritative query projection state.

### 5.4 Atomic Canonical Append

```text
validated AuditEvidenceEnvelope
        ↓
deterministic evidence commitment
        ↓
resolve Tenant + integrity epoch + integrity stream
        ↓
read current stream head + concurrency precondition
        ↓
construct predecessor linkage
        ↓
B02 OWNER-LOCAL ATOMIC TRANSACTION
        ├── persist canonical evidence
        ├── persist integrity-link / epoch metadata
        └── compare-and-advance IntegrityStreamHead
        ↓
COMMIT
        ↓
EvidencePersistenceReceipt
```

Canonical evidence, predecessor linkage and stream-head advancement form one logical commit boundary.

A stream-head conflict shall fail the attempted commit rather than create an unnoticed fork. Retry/re-evaluation uses the newly committed head according to B02-owned concurrency mechanics.

No distributed transaction across D03, C02 or other owners is introduced.

### 5.5 Store-Level Idempotency

D04-001 remains owner of audit-ingestion deduplication. D04-003 additionally guarantees safe persistence retry after uncertain execution outcome.

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

At that point D04-003 returns a conceptual `EvidencePersistenceReceipt` carrying:

- canonical evidence reference;
- integrity epoch/stream/position reference;
- committed integrity reference.

D04-001 may then perform its canonical `Audit Evidence Generated` publication handoff according to its locked architecture.

Periodic checkpoint sealing is a subsequent integrity-strengthening control and is not required to make every individually committed evidence record canonical.

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

The sequence supports detection of evidence-content modification, deletion from inside an otherwise retained sequence, reordering, predecessor substitution and undetected append-fork attempts when combined with head concurrency control.

The exact cryptographic construction remains D03-governed.

### 5.8 Checkpoint Sealing, Anchoring & Integrity Epochs

```text
committed integrity stream range / epoch boundary
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

The anchor boundary shall be logically protected from ordinary evidence write authority such that deleting or rolling back evidence and colocated checkpoint state cannot silently recreate a valid anchored state.

This specification does not require blockchain, a separate physical server, a specific HSM/object store/cloud service or a particular algorithm.

Evidence appended after the latest anchored checkpoint forms an **open tail**. D04-003 shall not claim identical rollback resistance for the open tail and an anchored closed range.

When stream topology changes, the previous integrity epoch must close through an anchored checkpoint before the successor assignment version/epoch becomes authoritative. Historical stream assignments remain immutable.

Checkpoint frequency and acceptable open-tail exposure are governed deployment/security parameters rather than invented numeric values.

### 5.9 Tamper-Detection Coverage

| Condition | Detection Mechanism |
|---|---|
| Evidence content modification | Canonical evidence commitment mismatch. |
| Middle-record removal | Sequential continuity failure. |
| Reordering | Sequential predecessor/position verification failure. |
| Concurrent fork attempt | B02 stream-head concurrency conflict. |
| Closed-range rollback | Checkpoint/independent-anchor disagreement. |
| Invalid topology/epoch transition | Missing/invalid predecessor epoch anchor or assignment-version continuity. |
| Query projection corruption | Canonical re-resolution and verification. |
| Export alteration | Export evidence/proof verification failure. |
| Recovery discontinuity | `RecoveryIntegrityGate` continuity verification failure. |

Integrity verification reports findings. It shall not rewrite evidence to force a valid result.

### 5.10 Canonical Evidence vs Query Projection

```text
CanonicalEvidenceRepository
        =
authoritative Audit Evidence

EvidenceQueryProjection
        =
derived / rebuildable / non-authoritative
```

The query projection may contain searchable references and approved derived attributes required for evidence discovery. It shall not become an alternate canonical evidence repository.

### 5.11 Projection Watermark and Reconciliation

Each projection maintains a technical watermark identifying the last canonical integrity position successfully incorporated.

If the projection is known to lag:

- the search result shall not be represented as complete beyond its watermark;
- exact canonical evidence retrieval remains possible where the canonical reference is known and authorization allows it;
- rebuild/reconciliation may occur from canonical evidence;
- canonical evidence shall not be modified to match a projection.

Projection rebuild converges from canonical evidence/integrity positions rather than treating existing projection contents as authority.

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

Authorization to query evidence does not grant authority to mutate evidence, stream heads, checkpoints, epochs or anchors.

### 5.12A Immutable Integrity Verification Result Preservation

`L4-A-024-BH-006`, `BH-008` and `BH-009` require the verification outcome itself to remain durable and immutable rather than existing only as a transient runtime response.

```text
authorized verification scope
        ↓
EvidenceIntegrityVerifier
        ↓
verification outcome + findings
        ↓
IntegrityVerificationResultRecorder
        ↓
B02 owner-local durable persistence
        ↓
immutable Integrity Verification Result reference
```

Each durably preserved `Integrity Verification Result` shall retain, at minimum, technical references sufficient to identify:

- the authorized verification scope;
- the canonical evidence references evaluated;
- the applicable integrity stream/epoch/checkpoint/anchor context where relevant;
- the verification time/context supplied by upstream authority;
- the verification outcome and detected inconsistencies/findings.

The result is an immutable audit outcome. It shall not rewrite, repair, relabel or replace canonical Audit Evidence.

Persistence of the result uses B02-owned physical persistence mechanics. Cryptographic protection, when required, uses D03-owned operations. This specification does not invent the publication event described by non-authorized L4 behavior outside the Stage-3 BH scope.

A verification execution for which durable-result preservation is required shall not be represented as durably complete if the result record fails to persist.

### 5.12B Immutable Investigation Evidence Package Reference Manifest

`L4-A-023-BH-005` requires an immutable `Evidence Package` that references canonical evidence. D04-003 realizes the durable audit-specific reference boundary without becoming owner of generic file packaging.

```text
authorized investigation scope
        ↓
canonical evidence references
        +
applicable Integrity Verification Result references
        +
applicable proof/checkpoint/anchor references
        ↓
EvidencePackageReferenceAssembler
        ↓
immutable Evidence Package reference manifest
```

The `Evidence Package` reference manifest shall:

- identify its authorized investigation scope;
- reference, rather than duplicate, canonical Audit Evidence;
- retain the evidence identities/digests and relevant integrity/proof references needed to verify what was included;
- retain applicable immutable `Integrity Verification Result` references where verification formed part of package assembly;
- remain immutable after successful creation;
- not create a second canonical owner for Audit Evidence.

If the Evidence Package is later materialized into files, archives or transferred externally, reusable file/archive packaging, transport, large-transfer handling and generic delivery integrity remain `L5-C-03-008` responsibilities.

### 5.13 Immutable Evidence vs Storage / Availability Control

Canonical evidence content and its integrity commitment are separate from upstream-defined Storage and Availability lifecycle dimensions.

```text
Immutable Audit Evidence
        ≠
Storage / Availability control representation
```

Transitions such as:

```text
Active Storage → Archived Storage
Available → Restricted Access
Restricted Access → Restored Access
```

shall not rewrite the canonical `AuditEvidenceEnvelope` or invalidate its original content commitment.

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
verify canonical evidence identity + integrity + required proof metadata
        ↓
commit authorized Storage / Availability outcome
        +
required publication intent through canonical owner boundary
        ↓
only then:
archival execution considered complete
```

A physical copy/movement alone is not archival completion. A control-state transition without a valid archived evidence representation is also not archival completion.

Partial execution shall preserve the last valid governed outcome and shall not publish or claim successful archival.

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

D04-003 shall not invent restoration duration, expiry time, closure actor, closure command or closure event because those remain unresolved upstream.

### 5.16 Audit-Specific Export Architecture

D04-003 owns audit-specific export selection/proof semantics, not the reusable file-transfer platform.

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

The audit-specific export proof material shall bind delivered audit evidence to canonical evidence identities, canonical integrity references, applicable checkpoint/anchor/epoch proof references, authorized evidence scope and applicable representation/protection version references.

D04-003 shall not define archive format, compression, transport protocol, file chunking, generic large-transfer mechanics or generic delivery acknowledgement. Those remain `L5-C-03-008` responsibilities.

An exported copy is never a new canonical Audit Evidence owner.

### 5.17 Persistence and Proof Evolution

Persistence/schema/proof evolution shall preserve historical verifiability.

Changes shall not:

- rewrite historical Audit Evidence merely to conform to a new physical representation;
- destroy the ability to resolve the original integrity-representation or stream-assignment version;
- invalidate historical checkpoint/anchor/epoch references without a governed continuity mechanism;
- silently replace old proof history as though the replacement proof had always existed;
- reshard historical evidence in place merely to rebalance a Tenant.

Where a newer supplementary proof is required, it shall remain traceable to the preserved historical evidence/proof basis.

B02-004 owns physical migration mechanics. D04-003 owns the requirement that historical audit integrity remains verifiable through those migrations.

### 5.18 Recovery Integrity Gate

Backup/restore/DR execution belongs to `L5-E-03`.

Before recovered D04-003 storage reopens for canonical append:

```text
recovered canonical evidence
+
recovered integrity / epoch metadata
+
recovered checkpoint records
+
recovered anchor references
        ↓
verify latest applicable anchored checkpoint/epoch
        ↓
verify continuity to recovered stream head
        ↓
verify recovered IntegrityStreamHead
        ↓
RecoveryIntegrityGate
     ↙             ↘
verified            unresolved / failed
   ↓                       ↓
reopen append        keep canonical append closed
```

An implementation shall not silently start a new integrity chain or epoch to disguise unresolved recovery discontinuity.

Projection state may be rebuilt and does not block canonical recovery if canonical evidence/proof continuity has been established.

### 5.19 Logical Privilege Separation

The architecture requires logical separation between technical authorities for:

- canonical evidence append;
- checkpoint/seal/epoch-transition initiation;
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

The implementation mechanism remains D02/D03/runtime-governed. No specific human role vocabulary is introduced.

### 5.20 Operational Integrity Signals

D04-003 shall expose technical conditions sufficient for E05-owned observation of:

- canonical append failure;
- evidence-identity integrity conflict;
- stream-head concurrency conflict;
- sequential continuity failure;
- invalid/missing integrity-epoch transition evidence;
- checkpoint-sealing failure;
- checkpoint backlog/open-tail position;
- checkpoint-anchor failure;
- projection watermark lag;
- projection reconciliation failure;
- integrity-verification failure;
- integrity-verification-result persistence failure;
- Evidence Package reference-manifest assembly/persistence failure;
- archive verification/incompletion;
- restored-access integrity failure;
- export-proof generation/verification failure;
- recovery continuity failure.

No threshold, timeout, retry count, alert product or SLA value is defined here.

---

## 6. Architecture & Technology Decisions

| ID | Decision | Selected Position | Basis | ADR |
|---|---|---|---|---|
| AD-001 | Canonical/query topology | One authoritative immutable evidence repository + rebuildable non-authoritative query projection. | L4-A-022/023; D04 boundary | `ADR-L5-0012` |
| AD-002 | Integrity topology | Sequential integrity linkage + periodic sealed checkpoints. | L4-A-024; D04 responsibility | `ADR-L5-0012` |
| AD-003 | Closed-range rollback protection | Sealed checkpoints require an independently protected anchor boundary. | Tamper-evidence responsibility | `ADR-L5-0012` |
| AD-004 | Tenant-local stream topology | One or multiple Tenant-local streams using versioned deterministic assignment and immutable integrity epochs. | Scale + historical verification | `ADR-L5-0012` |
| AD-005 | Topology evolution | Stream-assignment changes close the prior epoch through anchored checkpoint continuity and never rewrite historical assignment. | Historical proof preservation | `ADR-L5-0012` |
| AD-006 | Append consistency | Canonical evidence + integrity link + stream-head advancement commit atomically through B02. | B02-003 / `ADR-L5-0006` | Existing ADR |
| AD-007 | Persistence retry | Same evidence identity + same commitment is idempotent; same identity + different commitment is integrity conflict. | D04 persistence correctness | Local |
| AD-008 | Commitment representation | Integrity input is deterministic and explicitly version-bound. | Historical verification necessity | Local |
| AD-009 | Evidence/control separation | Immutable evidence content is separate from mutable authorized Storage/Availability control representation. | L4-A-026/027 | Local |
| AD-010 | Persistence milestone | Append/link/head commit is sufficient for D04-001 persistence receipt; checkpoint is subsequent strengthening. | Locked D04-001 CR-007 boundary | Local |
| AD-011 | Projection completeness | Query projection carries a canonical-position watermark and cannot claim completeness beyond it. | L4-A-023 investigation correctness | Local |
| AD-012 | Recovery reopening | Canonical append remains closed until recovered integrity continuity is verified. | D04 tamper-evident responsibility | Local |
| AD-013 | Export boundary | D04-003 owns audit-specific evidence/proof semantics; C03-008 owns generic packaging/transfer/delivery. | Frozen corpus responsibility | Local boundary decision |
| AD-014 | Privilege separation | Append, read/export and key-custody authorities remain logically separated. | `ADR-L5-0001/0002` | Existing ADRs |
| AD-015 | Verification-result durability | Completed integrity verification outcomes required by L4-A-024 are preserved as immutable durable `Integrity Verification Result` artifacts referencing canonical evidence/proof scope; they never repair evidence. | L4-A-024-BH-006/008/009 | Local realization |
| AD-016 | Evidence Package durability | Investigation `Evidence Package` is an immutable D04 reference manifest over canonical evidence/proof references; generic file/archive materialization remains C03-008-owned. | L4-A-023-BH-005; frozen C03-008 boundary | Local realization |

No database, search engine, object store, cryptographic algorithm, KMS/HSM, cloud provider, runtime or deployment product is selected.

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Architectural Impact | Owner / Collaborator |
|---|---|---|
| Tenant isolation | Integrity streams/checkpoints/queries never silently cross Tenant authority. | A04 / D04 |
| Stream scaling | Scale-out occurs inside a Tenant through versioned integrity epochs without historical rewrite. | D04 / B02 |
| Key custody | Raw key material remains outside D04-003. | D03 |
| Authorization | Access/export/restoration consumes D02 decisions and fails closed when absent. | D02 |
| Read vs write authority | Query/export capability does not imply canonical mutation authority. | D02 / D04 |
| Verification result integrity | Verification outcomes are immutable durable audit artifacts and cannot be used to rewrite canonical evidence. | D04 / B02 / D03 |
| Evidence Package integrity | Investigation packages remain immutable reference manifests to canonical evidence/proof scope; they do not duplicate evidence ownership. | D04 / C03 |
| Projection degradation | Search may degrade while canonical evidence remains valid. | D04 / E05 |
| Tamper rollback resistance | Anchored closed ranges have stronger rollback protection than the open tail. | D04 / D03 |
| Historical portability | Representation/proof/epoch versions remain resolvable through migration. | D04 / B02 |
| Recoverability | Canonical append reopens only after integrity continuity verification. | D04 / E03 |
| Export | Audit proof semantics stay D04-owned; generic delivery stays C03-008-owned. | D04 / C03 |

---

## 8. Architecture Conformance Requirements

| ID | Requirement |
|---|---|
| CR-001 | Canonical Audit Evidence shall not be updated in place after successful persistence. |
| CR-002 | Integrity input shall be deterministic and retain its integrity-representation version reference. |
| CR-003 | Integrity streams shall not cross Tenant authority boundaries. |
| CR-004 | Stream assignment shall use an explicit versioned deterministic assignment rule. |
| CR-005 | Historical evidence shall retain its original integrity epoch and stream assignment. |
| CR-006 | A topology change shall not become authoritative until the preceding epoch has the required anchored checkpoint continuity. |
| CR-007 | Historical evidence shall not be rewritten or resharded merely to rebalance integrity streams. |
| CR-008 | Canonical evidence, predecessor link and stream-head advancement shall commit atomically. |
| CR-009 | Concurrent stream-head change shall not create a silent fork. |
| CR-010 | Persistence retry of identical canonical evidence shall not create duplicate evidence. |
| CR-011 | Reuse of a canonical evidence identity with a different integrity commitment shall fail as an integrity conflict. |
| CR-012 | D04-003 shall return persistence success only after canonical evidence/link/head commit succeeds. |
| CR-013 | Periodic checkpoint sealing shall not be treated as a prerequisite for every `Audit Evidence Generated` publication. |
| CR-014 | A sealed checkpoint shall have an independently protected anchor before D04-003 claims closed-range rollback resistance. |
| CR-015 | D04-003 shall not claim identical rollback resistance for the unanchored open tail. |
| CR-016 | Integrity verification shall be read-only and shall surface rather than repair historical inconsistencies. |
| CR-017 | Query projections shall remain rebuildable and non-authoritative. |
| CR-018 | Query projections shall maintain a canonical-position watermark sufficient to identify known lag. |
| CR-019 | A known-lagging projection shall not be presented as complete beyond its watermark. |
| CR-020 | Canonical evidence shall be re-resolved from the authoritative repository before a projection result is treated as canonical evidence. |
| CR-021 | Archival shall not rewrite the original `AuditEvidenceEnvelope` or original evidence content commitment. |
| CR-022 | Archival completion shall not be claimed until archived representation integrity and required Storage/Availability outcomes have completed. |
| CR-023 | Restored archived access shall remain read-only and shall not change `Archived Storage`. |
| CR-024 | D04-003 shall not invent restoration duration, expiry, closure authority or closure event. |
| CR-025 | Audit-specific export proof material shall remain bound to canonical evidence identities and integrity references. |
| CR-026 | Generic export packaging/transfer/delivery mechanics shall remain C03-008-owned. |
| CR-027 | Persistence/proof migration shall preserve the ability to verify retained historical evidence and integrity-epoch history. |
| CR-028 | Recovery shall not reopen canonical append until applicable recovered integrity continuity is verified. |
| CR-029 | Recovery shall not silently start a replacement integrity chain/epoch to hide unresolved discontinuity. |
| CR-030 | Read/export authority shall not automatically include canonical evidence mutation authority. |
| CR-031 | D04-003 shall not own raw cryptographic key material. |
| CR-032 | Material D04 integrity failures shall be observable through E05-consumable technical signals. |
| CR-033 | A completed integrity verification for which L4-A-024 requires durable outcome preservation shall persist one immutable `Integrity Verification Result` referencing the authorized scope, canonical evidence evaluated, applicable proof context, outcome and detected inconsistencies; the result shall not mutate or repair canonical evidence. |
| CR-034 | An investigation `Evidence Package` shall be an immutable reference manifest to authorized canonical evidence and applicable proof/result references, shall not duplicate canonical Audit Evidence, and shall not absorb C03-008-owned generic file/archive packaging or transfer responsibilities. |

---

## 9. Open Decisions & Deferred Design

The former approval blockers are closed by `ADR-L5-0012` and `L5_Dhruv_Stage3_D04-003_Architecture_Approval_Decision_2026-09-12.md`.

| ID | Item | Classification | Approval Impact | Required Resolution |
|---|---|---|---|---|
| OD-001 | Checkpoint cadence/range and maximum acceptable open-tail exposure | Security / Operations parameter | Non-blocking for L5 Architecture Approval; required before production deployment | Establish governed security/operations values; do not invent them here. |
| OD-002 | Concrete persistence/search/object-storage technology | Technology | Non-blocking | Resolve through applicable B02/technology ADR governance if a product choice becomes material. |
| OD-003 | Restoration expiry/closure basis | Upstream | Non-blocking for core storage; blocks automatic closure semantics only | Resolve upstream `L4-A-027` gap. |
| OD-004 | Product-level routing/hash mechanism realizing deterministic stream assignment | Implementation / technology | Non-blocking | Must conform to `ADR-L5-0012`; do not make it business-semantic. |
| OD-005 | D04-002 stale ownership wording for physical DDL/storage engine, physical immutable-storage realization, cryptographic/Merkle sealing and reusable export packaging | Repository wording harmonization | **Closed** by `L5-D-04-002` v1.1.1 and `L5_D04-002_Ownership_Boundary_Correction_and_Relock_Decision_2026-09-13.md` | Canonical boundary is now explicit: B02 physical persistence; D03 cryptographic protection/key custody; D04-003 audit-specific integrity/proof semantics; C03-008 reusable packaging/transfer/delivery. |
| OD-006 | C03-008 exact handoff remains delegated to the canonical integration owner; no D04 ownership is implied. | Dependency | Non-blocking | Consume the current C03-008 contract when implementation requires the handoff; revalidate on material peer change. |

No blocking architecture decision remains for `L5-D-04-003`.

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-022-BH-007/009/010` — immutable canonical evidence and availability.
- `L4-A-023-BH-001/002/007/008` — authorized discovery/access.
- `L4-A-023-BH-005` — immutable investigation `Evidence Package` realized through `EvidencePackageReferenceAssembler`, `AD-016` and `CR-034`.
- `L4-A-024-BH-002/004/005` — read-only integrity verification and explicit integrity findings.
- `L4-A-024-BH-006/008/009` — immutable `Integrity Verification Result` preservation realized through `IntegrityVerificationResultRecorder`, `AD-015` and `CR-033`.
- `L4-A-025-BH-002–005/007` — retention/disposition authority.
- `L4-A-026-BH-001–008` — governed non-destructive archival.
- `L4-A-027-BH-001–007` — restored archived access and explicit closure gap.

These BHs are Stage-3-authorized behavioral authority for this specification by the governance decision dated 2026-09-12; their source L4 metadata remains unchanged.

### Primary L5 Dependencies

- `L5-D-04-001` — capture/admission/deduplication/publication boundary.
- `L5-D-04-002` — canonical evidence/provenance model; v1.1.1 explicitly preserves B02/D03/C03 ownership boundaries.
- `L5-B-02-003/004` — persistence concurrency, durable audit-artifact persistence and migration.
- `L5-D-03-001/002/003` — security, cryptography and keys.
- `L5-D-02` — authorization.
- `L5-A-04` — Tenant/Institution isolation.
- `L5-B-05-002/003` — disclosure and retention/disposition controls.
- `L5-C-03-008` — generic packaging/transfer/delivery.
- `L5-E-03` — recovery.
- `L5-E-05` — operational observation.

### Architecture Decision Authority

- `ADR-L5-0012` — canonical/query topology; Tenant-local versioned integrity streams; sequential linkage; checkpoints; independent anchoring; epoch transition/recovery continuity.
- `ADR-L5-0002` — cryptographic operation/key custody separation.
- `ADR-L5-0004` — Tenant/Institution isolation.
- `ADR-L5-0005/0006` — persistence ownership and concurrency.
- `ADR-L5-0008` — persistence evolution safety.
- `L5_D04-002_Ownership_Boundary_Correction_and_Relock_Decision_2026-09-13.md` — resolves the stale locked D04-002 ownership wording without changing D04-002 canonical responsibility.
- `L5_Dhruv_Stage3_D04-003_Traceability_Refinement_Approval_2026-09-13.md` — approves the explicit verification-result and Evidence Package realization refinements in v1.0.2.

### Governance Result

```text
Technical depth: PASS
Canonical ownership: PASS
Dependency boundaries: PASS
Security/trust boundaries: PASS
ADR governance: PASS
L4 behavioral-authority gate: PASS under narrow Stage-3 authorization
Integrity Verification Result traceability: PASS
Evidence Package traceability: PASS
Blocking architecture findings: 0
Status: Architecture Approved
Lock-ready: YES
Locked: NO — requires separate explicit final-governance authorization
```


## 2026-09-22 Batch 7B Impact Revalidation Synchronization

Non-material repository synchronization only. Current dependency metadata and/or authority wording is aligned to canonical governance without changing business authority, technical ownership, behavioral semantics, conformance requirements, traceability relation state, or lifecycle approval/lock basis.
