# L5-B-04-002 — Document Registry, Versioning, Storage & Retention Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-B-04-002 |
| Specification Title | Document Registry, Versioning, Storage & Retention Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-B — Data & Information Architecture |
| Major Area | L5-B-04 — Document, Media & Object Platform Architecture |
| Canonical Technical Responsibility | Technical document/object identity, immutable version lineage, storage realization and B05-authorized archival/disposition execution. |
| Technical Architecture Owner | Document Registry & Storage Technical Architecture |
| Assigned Author | Varad |
| Version | 1.1.0 |
| Status | Review |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-B-04-001; L5-B-02-001; L5-B-02-002; L5-B-02-003; L5-B-05-003; L5-D-03-001; L5-D-03-002; L5-D-04-001; L5-A-08-001; L4-C-018; L4-E-064; L4-H-004 |
| Referenced By | L5-B-04-003; L5-B-04-004; L5-B-04-005; domain document consumers |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0002; ADR-L5-0005; ADR-L5-0006; ADR-L5-0013; ADR-L5-0014 |

## 2. Purpose, Scope & Non-Goals

This specification defines the technical registry and storage realization for documents, media and objects. It owns technical identity, immutable version lineage and storage handoff. It executes archival or disposal actions only after receiving an authoritative disposition authorization from `L5-B-05-003`.

### In Scope

- Technical object/document identity and registry metadata.
- Immutable version lineage and current-version resolution.
- Storage reference abstraction and retrieval coordination.
- Integrity-reference association using D03-approved mechanisms.
- Archival-storage realization when authorized by B05.
- Disposal/storage-removal execution when explicitly authorized by B05 and permitted by canonical persistence/cryptographic owners.
- Recovery/reconciliation when registry and physical storage operations do not complete atomically.

### Out of Scope / Non-Goals

- Retention schedules, statutory durations or institutional retention policy.
- Legal-hold declaration, release or precedence decisions.
- Business disposal approval.
- Universal business lifecycle states such as ACTIVE/SUPERSEDED/ARCHIVED/PURGED.
- Default retention schedules when authoritative policy is absent.
- Cryptographic key destruction policy.
- Background scheduler/retry mechanics.
- Concrete object-store products, storage tiers, bucket layout or physical topology.

## 3. Canonical Ownership Boundary

`L5-B-05-003` is the locked owner of retention, archival, governed deletion and legal-hold controls. B04-002 therefore consumes, but never originates, these decisions.

| Concern | Canonical Owner | B04-002 Role |
|---|---|---|
| Retention schedule and duration | B05 / owning governance source | Consume resolved policy reference only. |
| Legal hold declaration/release and precedence | B05 | Consume effective hold/disposition decision; do not register or release holds. |
| Disposition authorization | B05 | Execute only the authorized technical action. |
| Archival storage realization | B04-002 | Own technical movement/reference update after B05 authorization. |
| Persistence mutation/deletion | B02 | Invoke governed persistence capability where registry records require mutation. |
| Cryptographic erasure | D03 | Invoke only when D03/B05 authorize and provide the capability. |
| Background scheduling/retry | A08 | Delegate work-unit execution; do not define retry policy locally. |
| Audit evidence | D04 | Emit/hand off technical evidence. |

Absence of an authoritative disposition instruction is a stop condition. This specification must not substitute an institutional default.

## 4. Architectural Invariants

1. Technical object identity must not replace upstream business identity.
2. Version lineage is append-only technical history where prior versions/evidence are required; replacement creates a new version rather than overwriting historical meaning.
3. Current-version resolution is a B04 technical convenience and must not be treated as an upstream business lifecycle state.
4. Registry/storage references must remain consistent enough to avoid exposing an object as successfully registered when its durable storage/reference state is unresolved.
5. Integrity evidence binds the exact technical version to which it applies.
6. Disposition executes only against the exact object/version scope authorized by B05.
7. Missing/stale/conflicting disposition authorization blocks action.
8. No local elapsed-time scheduler or retention default may originate disposition.

## 5. Technical Ownership

### 5.1 Owns

- Stable B04 technical object identity.
- Immutable technical version identity and predecessor lineage.
- Registry metadata necessary for locating/validating the technical object.
- Storage-reference abstraction independent of storage product.
- Current-version resolution for technical retrieval.
- Execution of B05-authorized archive/disposition storage actions.
- Reconciliation evidence for partially completed registry/storage operations.

### 5.2 Must Not Own

Business document identity where an upstream domain already owns it, business lifecycle state, business approval, retention policy, legal-hold state, deletion entitlement, key lifecycle, scheduler policy or physical storage-product topology.

## 6. Solution Architecture

### 6.1 Realization Components

| Element | Responsibility | Input | Output | Failure Boundary |
|---|---|---|---|---|
| `RegistryAdmissionBoundary` | Validate tenant/institution context, owner reference, correlation identity and artifact metadata required to register an object. | Registration request | Admitted registry request | Rejects ambiguous ownership or missing technical identity context. |
| `ObjectIdentityAllocator` | Establish stable B04 technical identity without replacing upstream business identity. | Admitted request | Technical object identity | Prevents duplicate technical registration where idempotency applies. |
| `VersionLineageManager` | Append immutable technical versions and link predecessors. | Existing object + new version metadata | New technical version identity | Historical versions are not overwritten by replacement. |
| `StorageReferenceManager` | Store opaque storage-location references returned by approved storage realization. | Artifact + context | Storage reference | Product-specific namespace semantics do not leak into canonical architecture. |
| `IntegrityReferenceBinder` | Associate D03-approved integrity evidence with an exact object version. | Version + D03 evidence | Integrity binding | Fails closed on version/evidence mismatch. |
| `DispositionExecutionBoundary` | Accept B05 disposition authorization and perform requested B04-owned storage action. | B05 authorization | Archive/removal result | Rejects absent, stale or conflicting authorization; never overrides legal hold. |
| `RegistryReconciliationBoundary` | Reconcile technical evidence when storage and registry updates complete partially/out of sequence. | Partial-operation evidence | Consistent technical registry/storage state | Cannot infer missing business disposition authority during recovery. |
| `RegistryEvidenceEmitter` | Produce technical evidence for register/version/archive/disposition/reconciliation operations. | Technical operation | Evidence handoff | Delegates canonical aggregation to D04. |

## 7. Conceptual Registry Contract

A registration/version request conceptually identifies tenant/institution, owning-domain/correlation reference, technical object reference when known, artifact/media metadata/profile, predecessor version reference where applicable and authorization/trust context.

A registry result conceptually returns technical object identity, technical version identity, predecessor/current-version relation, opaque storage reference, integrity reference where applicable, operation evidence reference and technical outcome.

A disposition execution request conceptually includes exact object/version scope plus authoritative B05 disposition authorization reference. Exact field schemas remain downstream design.

## 8. Technical Version & Retrieval Semantics

- Technical version lineage is not a business lifecycle.
- “Current” means the B04-selected technical retrieval pointer/reference and must not be interpreted as business-approved/current-issued status.
- Retrieval by exact version must remain distinguishable from retrieval of the B04 current-version reference.
- A replacement version does not inherit prior integrity evidence implicitly; integrity evidence must bind the exact version.
- If a referenced version is unavailable due to authorized disposition, B04 returns the technical availability outcome without inventing business consequences.

## 9. Registry–Storage Consistency & Recovery

Registry and physical storage operations may fail independently. The architecture therefore requires recoverable correlation/evidence rather than assuming a single atomic technology transaction across all boundaries.

- If storage succeeds but registry persistence fails, recovery uses durable technical operation evidence to reconcile rather than create an unrelated duplicate object.
- If registry intent exists but storage fails, the object/version must not be exposed as successfully available until the storage/reference state is resolved.
- Recovery must preserve object/version identity and predecessor lineage.
- Recovery must not invent B05 authorization or bypass D03/B02 ownership.

## 10. Retention / Archival / Disposal Flow

```text
B05 resolves policy + legal-hold precedence + disposition authorization
  -> B04-002 validates authorization and exact object/version scope
  -> if archive action: execute storage realization and update technical reference
  -> if removal action: coordinate required B02/D03 operation
  -> reconcile registry/storage state if partial failure occurs
  -> emit evidence to D04
  -> report execution result to B05 / owning workflow
```

B04-002 never initiates disposition merely because elapsed time or a local default has been reached.

## 11. Failure & Recovery Model

| Failure | Required Treatment |
|---|---|
| Missing B05 authorization | No disposition action. |
| Active/conflicting hold indicated by B05 | No removal; report blocked outcome. |
| Storage operation failure | Preserve existing valid registry/storage reference where possible; retry only through A08-governed execution. |
| Registry persistence failure after physical action | Reconcile from durable operation evidence; no silent lineage loss. |
| Integrity mismatch | Quarantine/reject affected version and notify owning workflow; no version substitution. |
| Duplicate registration/retry | Correlate idempotently where owner/correlation identity permits; do not create uncontrolled duplicate lineage. |
| Retrieval reference unavailable | Return explicit technical unavailable/unresolved outcome; do not infer business invalidity. |

## 12. Security, Privacy & Evidence

- Storage/retrieval access remains tenant/institution scoped and least privilege.
- Integrity operations use D03-approved capability; B04-002 does not select algorithms or manage private keys.
- Registry metadata should minimize sensitive content while retaining technical identity/lineage/evidence needs.
- Disposition evidence records authorization/correlation references without redefining B05 policy semantics.

## 13. Observability & Operability

Technical evidence should support correlation of object/version identity, predecessor/current relation, storage-reference transition, integrity binding, B05 disposition authorization reference, reconciliation outcome, failure category and D04 handoff state. Exact storage capacity, replication, lifecycle-tier or recovery timing targets remain downstream governed.

## 14. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-B04-002-01 | B04 owns technical object/version lineage; upstream domains retain business identity and business lifecycle. | Repository-derived boundary |
| AD-B04-002-02 | B04 executes archival/disposal storage actions only from B05 authorization. | Locked B05-003 ownership |
| AD-B04-002-03 | Historical technical versions are append-only lineage; replacement creates a new version. | Defensible technical derivation |
| AD-B04-002-04 | Registry/storage partial failures require evidence-based reconciliation without inferring business authority. | Technical consistency derivation |
| OD-B04-002-01 | Concrete object-store product, storage tiering, namespace layout and replication topology remain unresolved pending governed downstream design. | Open decision |

## 15. Conformance Requirements

1. No retention duration, legal hold or disposition entitlement shall be defined locally by B04-002.
2. Any archive/removal action shall carry authoritative B05 disposition authorization scoped to the exact object/version.
3. Missing retention policy shall not trigger a local/default retention schedule.
4. Technical versioning shall not introduce or overwrite upstream business lifecycle states.
5. Integrity evidence shall bind the exact technical version involved.
6. Registry/storage partial failure shall be recoverable through correlated technical evidence rather than silent duplication/loss.
7. Cryptographic erasure/key actions shall be delegated to D03.
8. Background scheduling and retry shall be delegated to A08.
9. Product-specific storage topology shall remain downstream unless approved by repository decision.

## 16. Architecture Acceptance Evidence

A conforming design must demonstrate stable object/version identity, no overwrite of historical lineage, exact-version integrity binding, B05-authorized disposition only, safe registry/storage partial-failure reconciliation, and no transfer of retention/legal-hold/key ownership into B04.

## 17. Traceability Summary

This architecture preserves `L5-B-05-003` as the authority for retention, legal hold and disposition while B04 owns document/object technical identity, version lineage and storage realization. B02 remains persistence-mechanics owner, D03 remains cryptography/key owner, A08 remains background-execution owner and D04 remains evidence-aggregation owner.