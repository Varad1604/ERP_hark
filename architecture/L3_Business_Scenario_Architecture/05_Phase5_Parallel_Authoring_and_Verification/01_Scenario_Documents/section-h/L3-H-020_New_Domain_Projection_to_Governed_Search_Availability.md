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
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H03 — Enterprise Directory & Search Workflows; Section A authorization and audit architecture; requesting canonical source-domain architecture  
**Referenced By:** Enterprise search onboarding, searchable projection governance, visibility governance, and audit activities

# 2. Scenario Purpose

Govern onboarding of a canonical source domain into enterprise search so its approved searchable projection contract can produce secure H03-owned projections without importing the source domain's business rules or ownership into H03.

# 3. Business Outcome

Its projection contract is approved, indexed, secured, and made discoverable without importing domain rules into H03

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a canonical domain requests enterprise search participation.

## 4.2 End Boundary

The frozen end boundary requires the domain's projection contract to be approved, indexed, secured, and made discoverable. Locked H03 defines contract registration, validation, projection lifecycle, security metadata, and discoverability, but does not define a canonical contract-approval lifecycle or approval authority. The complete end boundary is therefore unresolved.

## 4.3 In Scope

- Receiving a request from a canonical source domain to participate in enterprise search.
- Registering the domain's canonical indexing contract under H03 governance.
- Validating source identity, searchable attributes, required metadata, and contract conformance.
- Establishing H03-owned Searchable Projections with canonical references and security metadata.
- Progressing valid projections through H03 projection/index lifecycle toward searchable availability.
- Preserving source-domain business ownership and rule boundaries.

## 4.4 Out of Scope

- Importing source-domain business rules or lifecycle ownership into H03.
- Defining the missing contract approval authority or approval state locally.
- Defining technical index/storage/search-engine implementation.
- Changing source-domain entities to suit search requirements.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / Section A | Retains canonical authorization and audit ownership for governed administrative onboarding/approval activity. |
| Enterprise Directory & Search / Section H | Owns canonical indexing contract registration/validation, Searchable Projection lifecycle, search metadata, security metadata, and discoverability governance. |
| Requesting canonical source domain | Owns all canonical business entities, business rules, lifecycle decisions, source facts, and the authoritative meaning exposed for search. |

# 6. Business Actors, Roles and Responsibilities

The requesting canonical domain initiates search participation at domain level. H03's required Actors section is absent, and no canonical business actor or approval role is defined for approving a new projection contract. No approver is invented in this Draft.

# 7. Preconditions and Entry Criteria

- The requesting domain is a canonical owner of the business entities proposed for search participation.
- The domain can provide canonical identifiers and approved searchable attributes without transferring business ownership.
- H03 governance and Section A authorization/audit context are available.
- A canonical indexing contract can be registered and validated.

# 8. Trigger

A canonical domain requests enterprise search participation

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the canonical domain's request for enterprise search participation. | Requesting canonical domain / Enterprise Directory & Search | Onboarding scope and source ownership are identified. |
| 2 | Register the proposed canonical indexing contract under H03 governance. | Enterprise Directory & Search | The proposed projection relationship is formally represented without importing source rules. |
| 3 | Validate canonical identifiers, source ownership, searchable metadata, contract conformance, and required security metadata. | Enterprise Directory & Search / Section A | Only governed searchable information can proceed. |
| 4 | **Approval point unresolved:** determine whether the validated projection contract is approved for enterprise search participation. | **BLOCKED — authority not defined upstream** | The frozen mandatory approval outcome cannot be completed authoritatively. |
| 5 | After an upstream-approved contract exists, create governed H03 Searchable Projections and progress them through the defined projection lifecycle. | Enterprise Directory & Search | Search-side representations can be indexed without changing source ownership. |
| 6 | Make valid, secured projections discoverable under H03 visibility rules and preserve onboarding evidence. | Enterprise Directory & Search / Platform Audit | Governed enterprise search participation becomes available. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Contract validation fails | Proposed searchable attributes or metadata violate H03 contract rules. | Reject or return the contract for correction without indexing the source entity. | No invalid projection becomes searchable. | Enterprise Directory & Search |
| Security metadata incomplete | Projection cannot satisfy H03 visibility requirements. | Do not publish/make the projection searchable until required security metadata exists. | Search availability remains blocked. | Enterprise Directory & Search / Section A |
| Source-domain rule leakage detected | Proposed projection embeds source business logic. | Remove the rule leakage and preserve only approved searchable representation/metadata. | H03 remains a search/discovery domain rather than a competing business domain. | Source Domain / Enterprise Directory & Search |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Contract approval authority is unavailable | Mandatory frozen approval outcome cannot be determined. | Stop before approval and require upstream architecture clarification. | Enterprise Directory & Search / Section A |
| Projection validation fails after registration | Search onboarding cannot safely proceed. | Preserve the registration/validation evidence and keep the projection unavailable until corrected. | Enterprise Directory & Search |
| Canonical ownership becomes ambiguous | H03 risks becoming a competing source of truth. | Stop onboarding until source ownership is resolved upstream. | Requesting Source Domain / Enterprise Directory & Search |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| New searchable entity types must onboard through canonical indexing contracts without architectural modification. | Enterprise Directory & Search | Search participation requires a governed indexing contract. | L2-H03 BR-H03-040 / VR-H03-032 |
| Every projection references one canonical entity and identifies its source domain. | Enterprise Directory & Search | Search onboarding cannot transfer or duplicate source ownership. | L2-H03 BR-H03-002/003 |
| Projection schema must conform to canonical indexing contracts. | Enterprise Directory & Search | Invalid projection definitions cannot proceed. | L2-H03 VR-H03-005 |
| Security metadata must exist before publication/search availability. | Section A / Enterprise Directory & Search | New projections must be secured before discovery. | L2-H03 aggregate invariants / §12.1 |
| Business data modifications are prohibited within H03. | Enterprise Directory & Search | H03 cannot import source-domain rules or business transactions. | L2-H03 BR-H03-004 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Searchable Projection | No projection yet / `Detected` once source entity is identified | After a valid and **approved** contract, H03 can progress `Detected` → `Validated` → `Projected` → `Indexed` → `Searchable`. | `Searchable` | Enterprise Directory & Search | L2-H03 §§6.4, 12.1 |
| Canonical source entity | Existing source-owned lifecycle state | Referenced as source truth only. | Unchanged. | Requesting canonical source domain | L2-H03 ownership boundary |
| Canonical indexing contract | Proposed/registered contract | H03 defines registration and validation but no authoritative approval lifecycle/state. | **BLOCKED — approved state not defined** | Enterprise Directory & Search governance | L2-H03 Decision Matrix / VR-H03-005/032 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H03 defines `SearchProjectionRegistered` for a successfully created projection and `SearchIndexUpdated` for completed index updates. No canonical H03 event is defined for **projection contract approval**, so this L3 scenario does not invent one.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The mandatory hand-off is a canonical indexing contract that preserves source identity/ownership and declares permitted searchable attributes plus required security/synchronization metadata. The contract can be registered and validated under H03, but its frozen approval authority/lifecycle is unresolved upstream.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The canonical source domain initiates search participation. H03 requires privileged authorization for administrative operations, but no canonical business actor or approval authority is defined for approving a new indexing contract.

## 15.2 Separation of Duties, Overrides and Exceptions

No separation-of-duty or approval override may be invented. Contract validation cannot be silently treated as approval because the frozen outcome explicitly requires approval.

## 15.3 Business Audit and Evidence Obligations

Onboarding evidence must preserve requesting domain, canonical ownership references, proposed contract, validation results, security metadata, any eventual approval evidence once defined upstream, projection registration, resulting lifecycle state, and discoverability outcome. Platform Audit retains canonical audit-evidence ownership.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The frozen scenario cannot be fully completed until upstream architecture defines the projection-contract approval authority/lifecycle. After that resolution, completion additionally requires valid indexed projections, security metadata, searchable availability, and preserved source ownership.

## 16.2 Postconditions

- No source business entity or rule is owned by H03.
- No projection becomes discoverable before contract validation, required approval, and security governance are satisfied.
- The unresolved approval gap remains explicit and blocks architecture-complete completion.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The frozen outcome requires the projection contract to be **approved**, but L2-H03 defines registration, validation, onboarding, and administrative authorization without a canonical contract approval state, approval workflow, approver, or approval event. | A mandatory contract and authority step in the frozen final outcome is undefined. L3 cannot substitute validation for approval or invent an approver. | Define the canonical indexing-contract approval lifecycle, approval authority, and any required canonical event/evidence upstream. |
| L2-H03's Actors section is absent. | Even if approval is intended as an administrative action, no canonical business actor can be assigned. | Define H03 business actors/roles upstream. |

**Architecture assessment:** `BLOCKED`

The projection lifecycle, contract validation, security requirements, and discoverability are source-supported, but the mandatory frozen contract-approval outcome and authority are not. Upstream resolution is required.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
