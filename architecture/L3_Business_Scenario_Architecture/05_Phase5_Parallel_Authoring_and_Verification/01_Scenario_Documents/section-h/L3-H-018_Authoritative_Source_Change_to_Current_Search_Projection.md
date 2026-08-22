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
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H03 — Enterprise Directory & Search Workflows; applicable locked L2 source-domain architecture; Section A authorization and audit architecture  
**Referenced By:** Enterprise discovery, directory, search synchronization, operational verification, and audit activities

# 2. Scenario Purpose

Keep an H03-owned searchable projection current when its authoritative source domain publishes a searchable business fact or correction, while preserving the source domain as the sole owner of the canonical business entity and its lifecycle.

# 3. Business Outcome

The corresponding search projection becomes current and traceable without becoming a competing system of record

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authoritative source domain publishes a searchable business fact or correction under an approved H03 indexing/synchronization contract.

## 4.2 End Boundary

The scenario ends when the existing H03 Searchable Projection reflects the authoritative change, returns to its governed searchable position, and retains traceability to the source-domain identity and synchronization lineage.

## 4.3 In Scope

- Receiving a source-owned searchable change through the governed H03 collaboration boundary.
- Validating the source identity, projection contract, synchronization metadata, and visibility metadata required by H03.
- Updating the existing Searchable Projection without changing canonical source ownership.
- Restoring the projection to current searchable availability after successful synchronization.
- Publishing H03-owned projection/index completion events where defined.
- Preserving traceability to the authoritative source record and source version markers.

## 4.4 Out of Scope

- Modifying the authoritative source entity or its business lifecycle.
- Reinterpreting or correcting source-domain business meaning inside H03.
- Defining new source-domain event names or publishers.
- Defining search-engine, storage, transport, or technical synchronization mechanics.
- Recovering a corrupt or failed index, which is addressed by L3-H-019.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / Section A | Retains canonical authorization and audit ownership supporting governed H03 synchronization and traceability. |
| Enterprise Directory & Search / Section H | Owns the Searchable Projection, projection lifecycle, synchronization metadata, search visibility metadata, and H03 projection/index events. |
| VAR:Source-Owning Domain | Owns the canonical business entity, its business fact/correction, lifecycle, source event/contract, and authoritative identity. |

Cross-domain participation does not transfer ownership of the canonical business entity to H03.

# 6. Business Actors, Roles and Responsibilities

This scenario is initiated by an authoritative source-domain business fact rather than by a human actor. No additional H03 business actor is required for the normal synchronization path. Where an exceptional administrative intervention is needed, H03 requires authorization but does not define a canonical business actor catalogue; that gap is recorded in Section 17.

# 7. Preconditions and Entry Criteria

- The source domain is the authoritative owner of the business entity and change.
- An H03 Searchable Projection already represents the same canonical entity.
- The projection maintains the canonical identity reference and owning source-domain reference.
- The applicable indexing/synchronization contract is valid.
- Required synchronization and security/visibility metadata are available.

# 8. Trigger

A source domain publishes a searchable business fact or correction

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Accept the authoritative source change through the governed H03 synchronization boundary. | VAR:Source-Owning Domain / Enterprise Directory & Search | H03 receives a source-owned change without taking ownership of it. |
| 2 | Resolve the existing Searchable Projection by its immutable canonical reference and source-domain identity. | Enterprise Directory & Search | The correct H03 projection is identified. |
| 3 | Validate the change against the applicable canonical indexing contract and required synchronization/security metadata. | Enterprise Directory & Search | Only contract-valid, governed source information proceeds. |
| 4 | Apply the source change to the H03-owned searchable representation while preserving projection identity and source ownership. | Enterprise Directory & Search | The projection reflects the current authoritative fact. |
| 5 | Return the updated projection to governed searchable availability under existing visibility rules. | Enterprise Directory & Search | Authorized enterprise discovery uses the current projection. |
| 6 | Preserve synchronization lineage and publish applicable H03 completion events. | Enterprise Directory & Search / Platform Audit | The update is traceable to the source identity and H03 update outcome. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Contract validation fails | Source payload/change does not satisfy the canonical indexing contract. | Reject the projection update and preserve the prior valid searchable position. | No invalid source representation becomes current. | Enterprise Directory & Search |
| Source visibility changes | The authoritative change also changes visibility metadata supplied under the governed contract. | Re-evaluate H03 visibility metadata before restoring searchable availability. | Search visibility remains policy-compliant. | Enterprise Directory & Search / Section A |
| Source entity is archived or removed | The source lifecycle change is archival/removal rather than an ordinary update. | Follow the source-driven H03 archive/removal lifecycle instead of treating it as a normal update. | Search lifecycle follows source lifecycle without deleting the source entity. | Enterprise Directory & Search / Source-Owning Domain |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Projection update cannot be validated | Current source change cannot safely become searchable. | Retain the last valid projection state and route the failed synchronization under H03 recovery governance. | Enterprise Directory & Search |
| Projection becomes stale or inconsistent | Search no longer represents the authoritative source accurately. | Route to the governed index/projection recovery scenario L3-H-019. | Enterprise Directory & Search |
| Source ownership is unclear | H03 cannot safely determine the canonical business fact. | Stop the update and resolve canonical ownership upstream before changing the projection. | Source-Owning Domain / Enterprise Directory & Search |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| H03 shall never become the system of record for a business entity. | Enterprise Directory & Search | Projection updates cannot transfer business ownership. | L2-H03 BR-H03-001 |
| Every searchable projection references exactly one authoritative canonical entity and identifies its source domain. | Enterprise Directory & Search | Update remains tied to one canonical source. | L2-H03 BR-H03-002/003 |
| Projection metadata remains synchronized with authoritative sources. | Enterprise Directory & Search | Source changes must be reflected in the corresponding projection. | L2-H03 BR-H03-029 |
| Synchronization requests undergo contract validation before indexing. | Enterprise Directory & Search | Invalid source changes do not become current projections. | L2-H03 BR-H03-014 |
| Search ranking and visibility cannot override authorization. | Section A / Enterprise Directory & Search | Updated projections remain permission-aware. | L2-H03 BR-H03-017–024 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Searchable Projection | `Searchable` | Authoritative source update moves the projection to `Updated`; governed reindex/publication returns it to search availability. | `Searchable` after successful update/reindex. | Enterprise Directory & Search | L2-H03 §§6.4, 12.1 |
| Canonical source entity | Source-owned current lifecycle state | Referenced only as authoritative input. | Unchanged by H03. | VAR:Source-Owning Domain | L2-H03 ownership boundary |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

L2-H03 lists `CanonicalEntityUpdated` as a consumed event from **Any Authoritative Domain**, but this is a generic H03 consumption abstraction rather than one exact publisher for every source domain. This L3 scenario therefore does not assign or rename the upstream source event; the actual source-domain event/publisher must remain whatever that domain's locked architecture defines.

H03 may publish the exact H03-owned events `SearchProjectionUpdated` and `SearchIndexUpdated` after successful projection synchronization/index completion.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The source domain supplies canonical identity and approved searchable attributes through the governed canonical indexing/synchronization contract. H03 retains only the searchable projection and its synchronization/security metadata. No source business rule or lifecycle transfers to H03.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The authoritative source domain owns the business decision/fact that causes the change. H03 validates the resulting synchronization against its contract and visibility requirements. No additional H03 business approval is introduced for an ordinary source update.

## 15.2 Separation of Duties, Overrides and Exceptions

H03 administrative intervention requires authorization under Section A and H03 governance. No additional separation-of-duty rule is defined for the normal source-update path.

## 15.3 Business Audit and Evidence Obligations

Evidence must preserve the canonical entity reference, source domain, projection identity, prior/current synchronization lineage, source version marker where supplied, validation outcome, resulting projection state, and applicable H03 events. Platform Audit retains canonical audit-evidence ownership.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The authoritative source change has been contract-validated.
- The corresponding Searchable Projection reflects the current permitted source information.
- Projection identity and canonical source ownership remain unchanged.
- Governed security/visibility metadata remains valid.
- The projection is again available for authorized discovery.
- Synchronization lineage is traceable.

## 16.2 Postconditions

- H03 contains a current searchable representation, not a competing system of record.
- The source domain remains authoritative for the business entity and lifecycle.
- Applicable H03 update events are available for downstream H03 consumers.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-H03 names generic consumed events such as `CanonicalEntityUpdated` with producer `Any Authoritative Domain`, while actual source domains may publish different canonical event names. | L3 cannot assign one universal upstream event publisher without violating canonical event ownership. | Resolve the exact source event/publisher from each participating source domain when a concrete source projection is implemented/authored downstream. |
| L2-H03's canonical crosswalk records the Actors section as absent. | Exceptional administrative synchronization authority cannot be expressed as a named H03 business actor without invention. | Define canonical H03 business actors upstream if L3 needs named administrative authority. |

**Architecture assessment:** `PASS WITH ISSUES`

The frozen outcome is fully supportable from the H03 projection lifecycle, synchronization rules, ownership boundary, and published H03 update events. The open issues do not prevent a source-grounded normal update scenario.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
