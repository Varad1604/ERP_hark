# Enterprise Search Index Failure to Verified Recovery

# 1. Scenario Metadata

**Scenario ID:** L3-H-019  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Enterprise Directory & Search  
**Participating Domains:** A, H  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H03 — Enterprise Directory & Search Workflows; Section A authorization and audit architecture  
**Referenced By:** Enterprise search continuity, projection synchronization, operational governance, recovery verification, and audit activities

# 2. Scenario Purpose

Restore a missing, corrupt, stale, or failed H03 search index to a verified current state using authoritative canonical sources and H03 synchronization lineage, without modifying or taking ownership of source-domain business data.

# 3. Business Outcome

The affected index is rebuilt or replayed and reconciled with authoritative source checkpoints

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an H03 projection/index is missing, corrupt, stale, or fails processing such that enterprise search consistency cannot be trusted.

## 4.2 End Boundary

The scenario ends when the affected search index has been rebuilt from authoritative source information, validated against the governed source contract and synchronization/version markers, activated as the current searchable position, and recovery evidence is preserved.

## 4.3 In Scope

- Identifying the affected H03 index/projection scope.
- Entering the H03 governed recovery path after a critical index inconsistency/failure.
- Rebuilding the affected index from canonical source information and stored H03 synchronization metadata.
- Validating rebuilt content against source contracts and source version/synchronization markers.
- Restoring the index to a governed current searchable state.
- Publishing H03 recovery-completion events where defined.
- Preserving recovery evidence and prior search ownership boundaries.

## 4.4 Out of Scope

- Changing authoritative source-domain records.
- Defining implementation-level recovery mechanics.
- Changing source-domain event ownership.
- Introducing retry counts, queues, or technical recovery algorithms at L3.
- Onboarding a new source domain, which is addressed by L3-H-020.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / Section A | Retains authorization and canonical audit ownership for governed administrative recovery and evidence. |
| Enterprise Directory & Search / Section H | Owns the Search Index/Searchable Projection recovery lifecycle, synchronization metadata, validation, and H03 recovery events. |

Source-domain business data remains authoritative outside H03 even though the recovered index is reconstructed from canonical sources.

# 6. Business Actors, Roles and Responsibilities

Locked H03 requires explicit authorization for administrative rebuild/recovery but its required Actors section is absent. The normal failure/rebuild path is therefore described at domain level. No business actor is invented for exceptional administrative recovery.

# 7. Preconditions and Entry Criteria

- An H03 index or projection exists or is expected to exist for governed searchable scope.
- The index/projection is missing, corrupt, stale, or failed enough to require controlled recovery.
- Canonical source information and applicable indexing contracts remain available or recoverable.
- H03 retains synchronization metadata/source version markers sufficient to verify recovered consistency.
- Recovery does not require mutation of canonical source data.

# 8. Trigger

A projection is missing, corrupt, stale, or fails processing

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the affected H03 index/projection and classify the inconsistency as requiring controlled recovery. | Enterprise Directory & Search | Recovery scope is established without changing source ownership. |
| 2 | Preserve the existing projection identity, canonical references, and synchronization lineage needed for reconciliation. | Enterprise Directory & Search | Recovery remains traceable to the prior H03 state. |
| 3 | Rebuild the affected searchable representation from authoritative canonical source information under the applicable indexing contract. | Enterprise Directory & Search | A candidate recovered index/projection is produced from source truth. |
| 4 | Validate recovered content, security metadata, canonical references, and synchronization/source version markers. | Enterprise Directory & Search / Section A | The recovered representation is verified against governed source context. |
| 5 | Make the validated recovered index the current governed searchable position. | Enterprise Directory & Search | Enterprise search returns to a verified current state. |
| 6 | Publish the applicable H03 recovery completion event and preserve recovery evidence. | Enterprise Directory & Search / Platform Audit | Recovery is auditable and operationally traceable. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Incremental reconciliation is sufficient | Index inconsistency does not require full rebuild. | Apply the H03-governed synchronization/update path and validate the result. | Index returns to the governed current position without a full rebuild. | Enterprise Directory & Search |
| Rebuild validation fails | Rebuilt representation does not match the source contract or required metadata. | Do not activate the candidate recovery result; preserve the failed recovery evidence and escalate under H03 governance. | Invalid recovered state does not become searchable. | Enterprise Directory & Search |
| Canonical source unavailable | Required source truth cannot be obtained. | Keep the affected scope in governed recovery status until the authoritative source is available. | No competing or guessed source state is introduced. | Enterprise Directory & Search |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Recovery output cannot be validated | Search consistency remains unverified. | Preserve the previous valid position where available and require governed recovery follow-up. | Enterprise Directory & Search |
| Source contract violation is detected | Recovered projection cannot be trusted. | Reject the affected synchronization/recovery result and route to source/H03 governance. | Enterprise Directory & Search |
| Administrative recovery is required | Normal H03 recovery cannot complete. | Require authorized administrative recovery while preserving all prior evidence; actor remains undefined upstream. | Enterprise Directory & Search / Section A |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Search indexes are rebuildable from canonical sources. | Enterprise Directory & Search | Recovery must derive from source truth rather than a new business record. | L2-H03 BR-H03-005 / VR-H03-029 |
| Index corruption triggers rebuild of the affected index. | Enterprise Directory & Search | Full rebuild is an authoritative H03 recovery path. | L2-H03 Decision Matrix |
| Rebuild may only use canonical source and stored metadata, and rebuilt output must satisfy source contract validation before activation. | Enterprise Directory & Search | Recovered index must be verified before becoming current. | L2-H03 §§12.1, 12.3 |
| Recovery must protect canonical data and existing search contracts. | Enterprise Directory & Search | H03 recovery cannot mutate source-domain facts. | L2-H03 §12.3 |
| Administrative operations require explicit authorization and audit. | Section A / Enterprise Directory & Search | Exceptional recovery cannot be performed without governed authority/evidence. | L2-H03 Decision Principles / VR-H03-021/024 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Search Index | Any affected state / `Recovery Required` | Controlled recovery rebuilds and validates the affected index. | `Ready` after `Rebuilding` → `Activating` → `Ready`, where the defined rebuild path applies. | Enterprise Directory & Search | L2-H03 §12.3 |
| Searchable Projection | `Searchable`, `Updated`, `Rebuilt`, or inconsistent/missing expected projection | Projection is regenerated/reconciled from canonical source information. | `Searchable` after successful rebuild/publication. | Enterprise Directory & Search | L2-H03 §§6.4, 12.1 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H03 publishes the exact event `SearchIndexRebuilt` when full index rebuild completes. Where synchronization completion is part of reconciliation, H03 also defines `SearchSynchronizationCompleted`; failures may publish `SearchSynchronizationFailed`.

No source-domain event is republished as an H03-owned canonical business fact.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Recovery uses the existing canonical indexing contracts, canonical identity references, synchronization metadata, and source version markers already owned/recognized by H03. L3 does not define a new technical checkpoint mechanism.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Critical index failure may enter H03 recovery through its owned index lifecycle. Any administrative rebuild/recovery requires explicit authorization under H03/Section A governance; the specific H03 business actor is not defined upstream.

## 15.2 Separation of Duties, Overrides and Exceptions

No additional separation-of-duty rule is defined. Administrative recovery cannot override canonical source ownership or source-contract validation.

## 15.3 Business Audit and Evidence Obligations

Recovery evidence must preserve the affected index/projection identity, canonical references, pre-recovery condition, synchronization/source version markers, validation result, rebuild/reconciliation outcome, resulting state, and applicable H03 recovery events. Platform Audit retains canonical audit-evidence ownership.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The affected index/projection scope has been reconstructed from authoritative source information or reconciled through the governed H03 update path.
- Recovered content satisfies the canonical indexing contract and required security/synchronization metadata.
- The current H03 searchable position is verified against authoritative source lineage.
- Source ownership remains unchanged.
- Recovery evidence and applicable H03 completion events are preserved.

## 16.2 Postconditions

- Enterprise search uses the verified recovered H03 representation.
- H03 remains a projection/discovery owner only, not a system of record for source business entities.
- Any unresolved source-contract or administrative-recovery issue remains visible for governed follow-up.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The frozen outcome refers to reconciliation with authoritative source “checkpoints”; H03 defines synchronization metadata and source version markers but does not define a canonical business object literally named Source Checkpoint. | L3 can verify recovery against the source/version lineage defined by H03 but cannot invent a new checkpoint aggregate or lifecycle. | Clarify whether H03 synchronization/source version markers are the intended checkpoint semantics or define the canonical checkpoint concept upstream. |
| H03 requires explicit authorization for administrative recovery/rebuild but its Actors section is absent. | A named business actor for exceptional recovery cannot be assigned without invention. | Define canonical H03 business actors upstream if named administrative authority is required. |

**Architecture assessment:** `PASS WITH ISSUES`

H03 explicitly supports index inconsistency detection, rebuild from canonical sources, validation, activation, and `SearchIndexRebuilt`; the frozen recovery outcome can be achieved without inventing implementation mechanics. The checkpoint terminology and named administrative actor remain non-blocking upstream gaps.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
