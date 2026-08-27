# New Domain Projection to Governed Search Availability

# 1. Scenario Metadata

**Scenario ID:** L3-H-020  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Enterprise Directory & Search  
**Participating Domains:** A, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** L2-H03 — Enterprise Directory & Search Workflows; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows; `L2-H03_Search_Governance_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Govern onboarding of a canonical source domain into enterprise search so an approved projection definition can become securely available without importing source-domain business rules or ownership.

# 3. Business Outcome

Its projection contract is approved, indexed, secured, and made discoverable without importing domain rules into H03

The 2026-08-26 locked H03 amendment establishes the approval authority, projection lifecycle and canonical approval/activation events required for this frozen outcome.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A canonical domain requests enterprise search participation.

## 4.2 End Boundary

The scenario ends when the Search Projection Definition has passed governed approval, reached Active, and the resulting secure projection is available for governed enterprise discovery without changing source-domain ownership.

## 4.3 In Scope

- Search-participation request from a canonical source domain.
- H03 Search Projection Definition registration, review, approval and activation.
- Validation of source ownership, permitted projection scope, discoverability restrictions and institutional search policy.
- Preservation of source-domain ownership and rule boundaries.

## 4.4 Out of Scope

- Importing source-domain business rules into H03.
- Modifying source-domain entities or lifecycle states.
- Technical implementation of indexing or search infrastructure.

# 5. Ownership and Participating Domains

Section A retains authorization and audit capability ownership. H03 exclusively owns Search Projection Definitions, search configuration and H03 projection lifecycles. The requesting canonical source domain retains all source entities, rules, lifecycles and business meaning.

# 6. Business Actors, Roles and Responsibilities

The requesting source domain initiates participation at business-domain level. The **Search Governance Authority** is the canonical institutional business authority for approving or rejecting a new or materially changed Search Projection Definition before governed search availability. Technical administration does not substitute for this decision authority.

# 7. Preconditions and Entry Criteria

- The requesting domain is the canonical owner of the entities proposed for discovery.
- Canonical identifiers and permitted searchable attributes are available without ownership transfer.
- Section A authorization/audit context is available.
- A proposed Search Projection Definition can be evaluated against H03 governance.

# 8. Trigger

A canonical domain requests enterprise search participation

# 9. Main Business Flow

1. Receive the canonical source domain's search-participation request.
2. Establish a Draft Search Projection Definition under H03 governance.
3. Validate source-domain ownership, permitted projection scope, discoverability restrictions, security requirements and institutional search policy.
4. Move the definition to `Under Review` for the Search Governance Authority.
5. If approved, record `SearchProjectionApproved` and move the definition to `Approved`.
6. Activate the approved definition and publish `SearchProjectionActivated`.
7. Establish the governed searchable projection using only approved source attributes and references.
8. Make the resulting projection discoverable subject to H03 and Section A authorization constraints.
9. Preserve approval, activation, source ownership and projection evidence.

# 10. Alternate Business Flows

- `Under Review → Rejected`: publish `SearchProjectionRejected`; a later revision requires a successor Draft version and the rejected record remains historical.
- `Active → Suspended → Active`: temporary governed suspension does not change source-domain facts.
- `Suspended → Retired` or `Active → Retired`: publish `SearchProjectionRetired` and preserve history.
- Validation or security failure prevents approval/activation.

# 11. Failure, Interruption and Business Recovery

A projection that fails governance or security review does not become Active. Rejected definitions remain historical and may only be revised through a successor Draft. Suspension or retirement affects H03 projection availability only; source-domain data remains unchanged.

# 12. Business Rules and Decision Constraints

- H03 owns Search Projection Definitions and their lifecycle.
- Source domains retain ownership of source business facts.
- Search Governance Authority approves or rejects new or materially changed projection definitions.
- Projection scope must respect source-domain ownership, discoverability restrictions and institutional search policy.
- Technical administration cannot substitute for business approval.
- H03 does not import source-domain business rules.

# 13. State and Lifecycle Impact

Canonical Search Projection Definition lifecycle:

`Draft → Under Review → Approved → Active → Retired`

Branches:

- `Under Review → Rejected`
- `Active → Suspended → Active`
- `Suspended → Retired`

The source entity lifecycle remains unchanged.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H03 is the sole semantic publisher of:

- `SearchProjectionApproved`
- `SearchProjectionActivated`
- `SearchProjectionRejected`
- `SearchProjectionRetired`

These events describe H03 projection governance only and do not republish source-domain business facts.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The source domain supplies the authoritative entity references and permitted projection scope. H03 creates only search projections/definitions. Section A supplies authorization/audit capability without becoming owner of H03 decisions.

# 15. Business Authorization, Approval and Audit Obligations

Search Governance Authority owns the approval/rejection decision. Audit evidence preserves the participation request, source ownership, proposed definition, governance review, decision, activation/retirement outcome and applicable authorization context.

# 16. Completion Criteria and Postconditions

Completion requires:

- the projection definition is approved by the Search Governance Authority;
- the approved definition reaches Active;
- only approved searchable attributes are projected;
- discoverability restrictions and authorization remain effective; and
- source-domain ownership and rules remain unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing projection approval lifecycle, approval authority and approval event were resolved by locked `L2-H03_Search_Governance_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
