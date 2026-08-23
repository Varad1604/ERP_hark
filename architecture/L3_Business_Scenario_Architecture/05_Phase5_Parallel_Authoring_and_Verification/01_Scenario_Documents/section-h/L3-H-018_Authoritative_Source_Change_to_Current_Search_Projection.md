# Authoritative Source Change to Current Search Projection

# 1. Scenario Metadata

**Scenario ID:** L3-H-018  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Enterprise Directory & Search  
**Participating Domains:** A, H, VAR:Source-Owning Domain  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** L2-H03 — Enterprise Directory & Search Workflows; applicable authoritative source-domain L2; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Keep an H03 Searchable Projection current after an authoritative source-domain searchable fact or correction while ensuring H03 remains a projection/discovery owner rather than a competing system of record.

# 3. Business Outcome

The corresponding search projection becomes current and traceable without becoming a competing system of record

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A source domain publishes a searchable business fact or correction

## 4.2 End Boundary

The corresponding H03 Searchable Projection reflects the current permitted source fact, is again available for governed discovery, and retains source/synchronization traceability.

## 4.3 In Scope

- Authoritative source change consumption through H03 synchronization governance.
- Contract and search-specific validation.
- Update of H03-owned projection/search metadata.
- Restoration of governed search availability and synchronization lineage.

## 4.4 Out of Scope

- Changing the source entity or its business lifecycle.
- Reinterpreting source-domain business meaning inside H03.
- Index/projection corruption recovery, which is the H019 outcome.

# 5. Ownership and Participating Domains

Section A owns authorization/audit capabilities. H03 owns Searchable Projection lifecycle, synchronization metadata, search visibility metadata, and H03 projection/index events. `VAR:Source-Owning Domain` owns the canonical entity, source event/fact, business rules, and lifecycle.

# 6. Business Actors, Roles and Responsibilities

Normal synchronization is source-event driven and requires no new H03 human actor. H03 has no canonical Actors section for exceptional administrative synchronization, so no such role is invented.

# 7. Preconditions and Entry Criteria

- The source domain is authoritative for the business entity/change.
- A corresponding H03 Searchable Projection exists.
- Projection identity retains canonical source reference.
- Applicable indexing/synchronization contract is valid.
- Required security and synchronization metadata are available.

# 8. Trigger

A source domain publishes a searchable business fact or correction

# 9. Main Business Flow

1. Accept the authoritative source change through the governed H03 synchronization boundary.
2. Resolve the existing projection by canonical reference/source ownership.
3. Validate the change against the applicable indexing contract and required metadata.
4. Update only H03-owned searchable representation/metadata.
5. Re-establish governed searchable availability under current visibility rules.
6. Preserve synchronization lineage and publish applicable H03 projection/index completion events.

# 10. Alternate Business Flows

- **Contract validation failure:** retain the prior valid projection and do not publish an invalid update.
- **Visibility change:** re-evaluate H03 security/visibility metadata before search availability.
- **Source archive/removal:** follow the source-driven H03 archive/removal lifecycle rather than an ordinary update.

# 11. Failure, Interruption and Business Recovery

A stale/inconsistent projection is routed to H03 recovery rather than silently guessed. If source ownership is unclear, no projection update occurs until ownership is resolved upstream.

# 12. Business Rules and Decision Constraints

- H03 never becomes system of record for a source entity.
- Every projection references one canonical entity/source domain.
- Projection metadata remains synchronized with authoritative sources.
- Synchronization requests require contract validation before indexing.
- Search visibility cannot override authorization.

# 13. State and Lifecycle Impact

H03 projection lifecycle supports `Searchable → Updated` on canonical update and `Updated → Searchable` after successful reindex/update completion. The canonical source entity lifecycle is unchanged by H03.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H03 uses generic consumed source abstractions such as `CanonicalEntityUpdated`, but actual source domains may publish different canonical events. L3 does not assign a universal upstream publisher. H03 publishes `SearchProjectionUpdated` and `SearchIndexUpdated` for its own completed update facts where applicable.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The source domain supplies canonical identity and approved searchable attributes through the governed indexing/synchronization contract. H03 retains only the searchable projection and search-specific metadata.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The source domain owns the business fact causing the change. H03 validates the synchronization/search contract and visibility requirements; no additional business approval is introduced for an ordinary source update.

## 15.2 Separation of Duties, Overrides and Exceptions

Exceptional administrative synchronization requires authorization, but no additional separation-of-duty rule is defined.

## 15.3 Business Audit and Evidence Obligations

Preserve projection/source-reference and synchronization-lineage evidence required by H03. Platform Audit retains canonical audit-evidence ownership.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The source change is contract-valid for H03 search synchronization.
- The corresponding projection reflects current permitted source information.
- Security/visibility metadata remains valid.
- Search availability and synchronization lineage are current and traceable.

## 16.2 Postconditions

- H03 contains a current searchable representation, not a competing system of record.
- The source domain remains authoritative for the business entity and lifecycle.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H03's generic consumed event abstraction does not establish one exact upstream event/publisher for every source domain.
- H03's canonical Actors section is absent for exceptional administrative synchronization.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Resolve concrete source event/publisher mappings when a specific source-domain projection is authored.
