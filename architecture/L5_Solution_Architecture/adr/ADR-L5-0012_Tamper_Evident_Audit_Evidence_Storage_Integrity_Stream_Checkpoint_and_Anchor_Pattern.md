# ADR-L5-0012 — Tamper-Evident Audit Evidence Storage, Integrity Stream, Checkpoint & Anchor Pattern

**ADR ID:** `ADR-L5-0012`  
**Title:** Tamper-Evident Audit Evidence Storage, Integrity Stream, Checkpoint & Anchor Pattern  
**Scope:** Specification  
**Decision Owner:** `L5-D-04-003` — Tamper-Evident Audit Storage, Access & Export Architecture  
**Assigned Author:** Dhruv  
**Reviewers:** SAOS architecture review; Lead Architect approval in controlled Stage-3 architecture session  
**Status:** Architecture Approved  
**Decision Date:** 2026-09-12  
**Affected L5 Specifications:** `L5-D-04-003`  
**Related ADRs:** `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0004`; `ADR-L5-0005`; `ADR-L5-0006`; `ADR-L5-0008`  
**Supersedes:** None  
**Superseded By:** None

---

## 1. Decision Context

`L5-D-04-003` requires a product-neutral architecture that can make canonical Audit Evidence tamper-evident without transferring business-record ownership, cryptographic-key custody, physical persistence ownership, authorization policy or generic export-transfer responsibility into D04.

The specification requires decisions for:

- authoritative versus derived evidence storage;
- ordered integrity continuity;
- checkpoint sealing;
- rollback resistance;
- intra-Tenant integrity-stream partitioning;
- historical verification through topology evolution.

These choices materially affect security/trust, persistence consistency and multiple implementation boundaries and therefore require ADR governance.

## 2. Scope & Affected Architecture

This ADR governs only the audit-evidence integrity topology owned by `L5-D-04-003`.

It does not select:

- database/search/object-storage products;
- cryptographic algorithms, key sizes, KMS/HSM products or key custody;
- physical deployment topology;
- checkpoint cadence or numeric open-tail limit;
- generic file packaging/transfer mechanisms;
- business retention/disposition policy.

## 3. Authoritative Constraints

1. Canonical Audit Evidence remains immutable and source-business ownership is unchanged.
2. Query/index projections are derived and non-authoritative.
3. Integrity streams may not cross Tenant authority boundaries.
4. Canonical evidence, predecessor linkage and stream-head advancement use B02 owner-local atomic transaction/concurrency semantics.
5. Cryptographic operations belong to D03-002 and key custody/lifecycle belongs to D03-003.
6. Integrity verification is read-only and may report but not repair historical evidence.
7. Archived/restored evidence preserves canonical evidence identity and original evidence commitment.
8. Historical proof continuity must remain verifiable through persistence evolution.

## 4. Decision Criteria

- tamper detection;
- rollback resistance;
- deterministic historical verification;
- scalability without cross-Tenant integrity coupling;
- persistence correctness under concurrency;
- recovery verifiability;
- product neutrality;
- clear ownership boundaries;
- migration/evolution safety.

## 5. Alternatives Considered

### A — Single authoritative immutable store queried directly

Rejected as the sole topology because broad investigation/search needs would force canonical persistence to carry query responsibilities and scaling concerns that are better isolated in a rebuildable projection.

### B — Authoritative immutable store plus derived non-authoritative query projection

Selected.

Canonical evidence remains authoritative. Search/query structures may be rebuilt and may not create canonical evidence truth.

### C — Archive-first canonical storage with restore-before-investigation

Rejected as the general model because current canonical evidence must remain available for authorized investigation without treating archive restoration as the normal query path.

### Integrity alternative 1 — Sequential integrity linkage only

Rejected as insufficient for strong closed-range rollback detection when the latest evidence and its colocated integrity metadata could be removed together.

### Integrity alternative 2 — Periodic checkpoint sealing only

Rejected because it weakens per-record ordered continuity and leaves larger ambiguity inside checkpoint ranges.

### Integrity alternative 3 — Sequential linkage plus sealed checkpoints and independently protected checkpoint anchoring

Selected.

## 6. Evaluation

| Criterion | Direct-only canonical store | Canonical + projection | Archive-first |
|---|---|---|---|
| Canonical authority clarity | Strong | Strong | Strong |
| Broad query scalability | Weak/Moderate | Strong | Weak |
| Projection rebuildability | N/A | Strong | N/A |
| Investigation availability | Strong | Strong | Weak |
| Responsibility separation | Moderate | Strong | Moderate |

| Integrity criterion | Sequential only | Checkpoint only | Hybrid + anchor |
|---|---|---|---|
| Record-order continuity | Strong | Moderate | Strong |
| Middle-record tamper detection | Strong | Moderate | Strong |
| Closed-range rollback resistance | Weak/Moderate | Strong when independently anchored | Strong |
| Incremental verification | Strong | Moderate | Strong |
| Recovery verification | Strong | Strong | Strongest overall |

## 7. Decision

SutraERP shall use the following D04-003 integrity topology.

### 7.1 Canonical and Query Storage

1. One authoritative append-only Audit Evidence repository is the canonical evidence source.
2. Search/index/query projections are derived, rebuildable and non-authoritative.
3. Canonical evidence is re-resolved from the authoritative repository before a query result is treated as canonical evidence.
4. Query projections maintain a canonical-position watermark sufficient to identify known lag.

### 7.2 Sequential Integrity Linkage

1. Each canonical append participates in an ordered integrity stream.
2. Each committed evidence integrity value binds to the prior committed integrity value or an explicit stream-root condition.
3. Canonical evidence, integrity-link metadata and stream-head advancement commit atomically through B02 owner-local transaction semantics.
4. Concurrent stream-head change produces an explicit conflict and shall not create a silent fork.

### 7.3 Tenant-Local Versioned Stream Partitioning

1. No integrity stream may span multiple Tenant authority boundaries.
2. A Tenant may use one or multiple logical integrity streams.
3. Stream assignment for an evidence record is determined by a **versioned deterministic assignment rule** using only stable technical inputs authorized by D04/A04 architecture.
4. The exact product-level hash/routing mechanism is not selected by this ADR.
5. A stream-assignment version defines an **integrity epoch** for that Tenant.
6. Evidence already committed in an epoch never changes stream assignment merely because the partitioning strategy later evolves.
7. A topology/assignment-version change becomes effective only after the prior epoch is closed by an anchored checkpoint boundary sufficient to verify historical continuity.
8. The successor epoch records the prior anchored epoch reference required to preserve verifiable continuity across the topology change.
9. Implementations shall not reshard/rewrite historical evidence in place to rebalance integrity streams.

This permits scale-out without breaking historical verification or making one Tenant's evidence dependent on another Tenant's integrity chain.

### 7.4 Checkpoint Sealing and Anchoring

1. Bounded committed integrity-stream ranges are periodically sealed.
2. D03 supplies cryptographic protection; D04 does not own algorithms or raw keys.
3. A closed-range rollback-resistance claim requires an independently protected checkpoint-anchor reference.
4. The anchor must be logically protected from ordinary evidence-write authority so rollback of canonical evidence and colocated metadata cannot silently recreate a valid anchored state.
5. Evidence after the latest anchored checkpoint is an open tail and shall not be represented as having identical rollback resistance to an anchored closed range.
6. Numeric checkpoint cadence/range and acceptable open-tail exposure are governed operational/security parameters, not invented here.

### 7.5 Recovery and Evolution

1. Recovery shall verify the latest applicable anchored checkpoint, continuity to the recovered head and the recovered stream-head state before canonical append reopens.
2. Recovery shall not silently create a new chain to hide an unresolved discontinuity.
3. Persistence/schema evolution shall preserve historical evidence, integrity-representation version references, checkpoint references and epoch/topology history required for verification.

## 8. Consequences

### Positive

- Canonical evidence authority remains clear.
- Search scaling does not weaken evidence authority.
- Ordered tamper detection and stronger closed-range rollback detection coexist.
- Tenant boundaries are preserved.
- High-volume Tenants can scale across multiple integrity streams without rewriting history.
- Stream-topology evolution remains historically verifiable.
- Recovery can determine whether canonical append is safe to reopen.

### Cost / Complexity

- Implementations must maintain stream heads, checkpoint state, anchor references, projection watermarks and topology/epoch history.
- Multi-stream Tenants require deterministic assignment and verification across multiple stream histories.
- Independent checkpoint anchoring introduces an additional trust boundary.

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| One stream becomes a throughput hotspot | Permit multiple Tenant-local streams using versioned deterministic assignment. |
| Repartitioning destroys historical proof | Historical assignments are immutable; topology changes create a new anchored epoch. |
| Projection becomes mistaken for evidence authority | Projection remains explicitly non-authoritative and rebuildable. |
| Checkpoint and evidence are rolled back together | Require independently protected anchor before claiming closed-range rollback resistance. |
| D04 absorbs cryptographic/key ownership | D03-002/003 remain canonical owners. |
| Product-specific routing leaks into architecture | ADR defines deterministic/versioned semantics only, not routing technology. |

## 10. Migration / Exit Position

The architecture may move between database, search, object-storage, cryptographic or deployment products without superseding this ADR if the decision invariants remain satisfied.

A future architecture that removes the canonical/query split, sequential integrity continuity, independent closed-range anchoring, or versioned Tenant-local stream epochs requires explicit ADR supersession.

## 11. Traceability

Primary realization: `L5-D-04-003_Tamper-Evident_Audit_Storage_Access_and_Export_Architecture.md`.

Key upstream behavior:

- `L4-A-022-BH-007/009/010`;
- `L4-A-023-BH-001/002/005/007/008`;
- `L4-A-024-BH-002/004/005/006/008/009`;
- `L4-A-026-BH-001–008`;
- `L4-A-027-BH-001–007`.

Key technical dependencies:

- `ADR-L5-0002` — cryptographic operations/key-custody separation;
- `ADR-L5-0004` — Tenant/Institution isolation;
- `ADR-L5-0005/0006` — persistence ownership and transaction/concurrency;
- `ADR-L5-0008` — compatibility-safe persistence evolution.

## 12. Approval Record

**Current disposition:** Architecture Approved.  
**Approval basis:** The selected topology is product-neutral, preserves canonical ownership, closes the D04-003 integrity-stream partitioning decision, and was explicitly approved by the Lead Architect in the controlled Stage-3 architecture session on 2026-09-12. This approval does not imply `Locked` status.

| Role | Decision | Date | Evidence |
|---|---|---|---|
| D04-003 technical architecture review | Recommended | 2026-09-12 | Technical-depth and hard-fail review |
| SAOS architecture reviewer | PASS | 2026-09-12 | D04-003 production architecture review |
| Lead Architect / architecture promotion authorization | Approved | 2026-09-12 | Explicit approval in controlled Stage-3 architecture session |
