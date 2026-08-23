# New Domain Projection to Governed Search Availability

# 1. Scenario Metadata

**Scenario ID:** L3-H-020  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Enterprise Directory & Search  
**Participating Domains:** A, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** L2-H03 — Enterprise Directory & Search Workflows; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Govern onboarding of a canonical source domain into enterprise search so an approved indexing/projection contract can produce secure H03 Searchable Projections without importing source-domain business rules or ownership.

# 3. Business Outcome

Its projection contract is approved, indexed, secured, and made discoverable without importing domain rules into H03

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A canonical domain requests enterprise search participation

## 4.2 End Boundary

The frozen outcome requires the projection contract to be approved, indexed, secured, and made discoverable. Locked H03 does not define the mandatory contract-approval lifecycle or authority, so the scenario cannot authoritatively reach this end boundary.

## 4.3 In Scope

- Search-participation request from a canonical source domain.
- H03 indexing/projection-contract registration and validation.
- Searchable Projection lifecycle and required security metadata.
- Preservation of source-domain ownership and rule boundaries.

## 4.4 Out of Scope

- Importing source-domain business rules into H03.
- Inventing the missing contract approver, approval lifecycle, or approval event.
- Changing source-domain entities to satisfy search participation.

# 5. Ownership and Participating Domains

Section A retains authorization and Audit ownership for governed onboarding. H03 owns indexing-contract registration/validation, Searchable Projection lifecycle, search/security metadata, and discoverability. The requesting canonical source domain retains all business entities, rules, lifecycle decisions, and source meaning.

# 6. Business Actors, Roles and Responsibilities

The requesting domain initiates at domain level. H03 has no canonical Actors section and defines no canonical business approver for a new indexing/projection contract. No approver is invented.

# 7. Preconditions and Entry Criteria

- The requesting domain is canonical owner of the entities proposed for discovery.
- It can provide canonical identifiers and permitted searchable attributes without transferring ownership.
- H03 governance and Section A authorization/audit context are available.
- A canonical indexing contract can be registered and validated.

# 8. Trigger

A canonical domain requests enterprise search participation

# 9. Main Business Flow

1. Receive the canonical domain's search-participation request.
2. Register the proposed indexing/projection contract under H03 governance.
3. Validate canonical identifiers, source ownership, permitted searchable attributes, contract conformance, and required security metadata.
4. **Blocking decision:** establish contract approval before indexing. Locked H03 requires approved indexing contracts but defines no approval lifecycle, approver, or approval event.
5. After a future authoritative approval exists, H03 can create projections and progress them through `Detected → Validated → Projected → Indexed → Searchable`.
6. Make only secured, valid projections discoverable and preserve source ownership/evidence.

# 10. Alternate Business Flows

- **Contract validation fails:** return/reject without search availability.
- **Security metadata incomplete:** do not make the projection searchable.
- **Source rule leakage detected:** retain only search representation/metadata; source business rules stay source-owned.

# 11. Failure, Interruption and Business Recovery

Without canonical contract approval authority, onboarding stops before the mandatory approval decision. Validation failure or ownership ambiguity likewise prevents search availability; no local approval substitute or source-ownership compromise is permitted.

# 12. Business Rules and Decision Constraints

- New searchable entity types integrate through canonical indexing contracts.
- Search projections contain only approved searchable attributes.
- Every projection references exactly one canonical entity and source domain.
- Security metadata must exist before indexing/publication.
- H03 may not modify source business data or import source business rules.

# 13. State and Lifecycle Impact

After an approved contract, H03 can use the source-defined Projection lifecycle `Detected → Validated → Projected → Indexed → Searchable`. The canonical source entity remains unchanged. No canonical indexing-contract approval state machine exists in locked H03.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H03 defines `SearchProjectionRegistered` and `SearchIndexUpdated` for projection/index outcomes, but no canonical event for projection-contract approval. No approval event or publisher is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The requesting domain supplies the proposed canonical indexing/projection contract and remains owner of all source business meaning. H03 can register and validate the contract, but the mandatory approval hand-off/authority is undefined upstream.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The canonical source domain initiates search participation. H03 requires governed administrative authorization but defines no canonical business actor/approval authority for approving the indexing/projection contract.

## 15.2 Separation of Duties, Overrides and Exceptions

Validation cannot be silently treated as approval, and no separation-of-duty or override rule may be invented.

## 15.3 Business Audit and Evidence Obligations

Preserve the onboarding request, source ownership, proposed contract, validation/security results, eventual approval evidence once defined upstream, and H03 projection availability evidence.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A canonical approval lifecycle/authority exists for the projection contract.
- The contract is validly approved.
- H03 projections are indexed and secured.
- The projections become discoverable without importing source-domain rules.

## 16.2 Postconditions

- No architecture-complete postcondition can be asserted until the mandatory contract-approval gap is resolved.
- Source business ownership remains outside H03.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- Frozen semantics require an **approved** projection contract, but locked H03 defines no approval state, workflow, business approver, or approval event.
- H03 has no canonical Actors section for the mandatory contract-approval action.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream architecture must define the indexing/projection-contract approval lifecycle, authority, and any required event/evidence.
