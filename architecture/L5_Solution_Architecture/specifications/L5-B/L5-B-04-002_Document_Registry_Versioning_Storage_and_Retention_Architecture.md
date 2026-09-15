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
| Version | 1.0.1 |
| Status | Locked |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-B-04-001; L5-B-02-001; L5-B-02-002; L5-B-02-003; L5-B-05-003; L5-D-03-001; L5-D-03-002; L5-D-04-001; L5-A-08-001; L4-C-018; L4-E-064; L4-H-004 |
| Referenced By | L5-B-04-003; L5-B-04-004; L5-B-04-005; domain document consumers |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0002; ADR-L5-0005; ADR-L5-0006; ADR-L5-0013; ADR-L5-0014 |

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the technical registry and storage realization for documents, media and objects. It owns technical identity, immutable version lineage and storage handoff. It executes archival or disposal actions only after receiving an authoritative disposition authorization from `L5-B-05-003`.

### In Scope

- Technical object/document identity and registry metadata.
- Immutable version lineage and current-version resolution.
- Storage reference abstraction and retrieval coordination.
- Integrity-reference association using D03-approved mechanisms.
- Archival-storage realization when authorized by B05.
- Disposal/storage-removal execution when explicitly authorized by B05 and permitted by the canonical persistence/cryptographic owners.

### Out of Scope / Non-Goals

- Retention schedules, statutory durations or institutional retention policy.
- Legal-hold declaration, release or precedence decisions.
- Business disposal approval.
- Universal business lifecycle states such as ACTIVE/SUPERSEDED/ARCHIVED/PURGED.
- Default retention schedules when authoritative policy is absent.
- Cryptographic key destruction policy.
- Background scheduler/retry mechanics.
- Concrete object-store products, storage tiers, bucket layout or physical topology.

---

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

---

## 4. Technical Ownership

### 4.1 Owns

- `DocumentObjectId` as the canonical technical identity for a B04-managed object.
- Immutable `DocumentVersionId` lineage and predecessor relationship.
- Registry metadata necessary for locating and validating the technical object.
- Storage-reference abstraction independent of storage product.
- Current-version resolution for technical retrieval.
- Execution of B05-authorized archive/disposition storage actions.

### 4.2 Must Not Own

Business document identity where an upstream domain already owns it, business lifecycle state, business approval, retention policy, legal-hold state, deletion entitlement, key lifecycle, or scheduler policy.

---

## 5. Solution Architecture

### 5.1 Realization Components

| Element | Responsibility | Input | Output | Failure Boundary |
|---|---|---|---|---|
| `RegistryAdmissionBoundary` | Validates tenant/institution context, owner reference and artifact metadata required to register an object. | Registration request | Admitted registry request | Rejects ambiguous ownership or missing technical identity context. |
| `ObjectIdentityAllocator` | Establishes stable B04 technical identity without replacing upstream business identity. | Admitted request | `DocumentObjectId` | Prevents duplicate technical registration for the same governed correlation where idempotency applies. |
| `VersionLineageManager` | Appends immutable technical versions and links them to predecessors. | Existing object + new version metadata | New `DocumentVersionId` | Historical versions are never overwritten by replacement. |
| `StorageReferenceManager` | Stores opaque storage-location references returned by the approved storage realization. | Artifact + context | Storage reference | Does not expose product-specific bucket/container semantics as canonical architecture. |
| `IntegrityReferenceBinder` | Associates D03-approved integrity evidence with an exact object version. | Version + D03 evidence | Integrity binding | Fails closed on version/evidence mismatch. |
| `DispositionExecutionBoundary` | Accepts B05 disposition authorization and performs the requested B04-owned storage action. | B05 authorization | Archive/removal result | Rejects absent, stale or conflicting authorization; never overrides a legal hold. |
| `RegistryEvidenceEmitter` | Produces technical evidence for register/version/archive/disposition operations. | Technical operation | Evidence handoff | Delegates audit aggregation to D04. |

### 5.2 Technical Version Model

B04-002 uses technical version lineage, not a new business lifecycle. A version can be described technically as current/non-current or available/unavailable for retrieval when required, but these labels must not be interpreted as business document states. Business status remains with the owning domain.

### 5.3 Storage Abstraction

The registry stores an opaque storage reference plus integrity and media metadata needed to retrieve the object through the approved B04 storage boundary. Concrete storage products, tier names, replication schemes and physical namespace layout remain implementation decisions unless separately governed.

### 5.4 Retention / Archival / Disposal Flow

```text
B05 resolves policy + legal-hold precedence + disposition authorization
  -> B04-002 validates authorization and exact object/version scope
  -> if archive action: execute storage realization and update technical reference
  -> if removal action: coordinate required B02/D03 operation
  -> emit evidence to D04
  -> report execution result to B05 / owning workflow
```

B04-002 never initiates disposition merely because elapsed time or a local default has been reached.

### 5.5 Failure & Recovery

- Missing B05 authorization: no disposition action.
- Active/conflicting hold indicated by B05: no removal; report blocked outcome.
- Storage operation failure: preserve existing valid registry/storage reference and retry only through A08-governed execution.
- Registry persistence failure after physical action: recovery must reconcile from durable operation evidence; no silent loss of lineage.
- Integrity mismatch: quarantine/reject affected version and notify owning workflow; do not silently substitute another version.

---

## 6. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-B04-002-01 | B04 owns technical object/version lineage; upstream domains retain business identity and business lifecycle. | Repository-derived boundary |
| AD-B04-002-02 | B04 executes archival/disposal storage actions only from B05 authorization. | Locked B05-003 ownership |
| AD-B04-002-03 | Historical technical versions are append-only lineage; replacement creates a new version. | Defensible technical derivation |
| OD-B04-002-01 | Concrete object-store product, storage tiering, namespace layout and replication topology remain unresolved pending governed downstream design. | Open decision |

---

## 7. Conformance Requirements

1. No retention duration, legal hold or disposition entitlement shall be defined locally by B04-002.
2. Any archive/removal action shall carry an authoritative B05 disposition authorization scoped to the exact object/version.
3. Missing retention policy shall not trigger a local/default retention schedule.
4. Technical versioning shall not introduce or overwrite upstream business lifecycle states.
5. Cryptographic erasure/key actions shall be delegated to D03.
6. Background scheduling and retry shall be delegated to A08.
7. Product-specific storage topology shall remain downstream unless approved by repository decision.

---

## 8. Traceability Summary

This revision resolves the prior B04/B05 ownership collision by making `L5-B-05-003` authoritative for retention, legal hold and disposition while preserving B04's canonical responsibility for document/object registry, technical version lineage and storage realization.