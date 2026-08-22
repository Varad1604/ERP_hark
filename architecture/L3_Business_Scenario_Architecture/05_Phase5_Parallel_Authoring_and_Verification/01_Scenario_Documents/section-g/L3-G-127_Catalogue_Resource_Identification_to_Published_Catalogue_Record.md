# Catalogue Resource Identification to Published Catalogue Record

**Scenario ID:** L3-G-127  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

This scenario governs the complete institutional process through which a library resource is identified, its bibliographic information is prepared and classified, the catalogue record is reviewed/approved, and the resource becomes a Published Catalogue Record.

The scenario establishes the intellectual-resource record. It does not create or register a physical library copy; physical copy management is a separate G05 aggregate and workflow.

## 3. Business Outcome

A library resource has a complete, reviewed and approved catalogue record and is Published as authoritative bibliographic information available to library operations and downstream consumers.

The published catalogue record can subsequently serve as the canonical intellectual-resource reference for physical-copy registration and circulation.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A library resource is identified as requiring catalogue registration.

### 4.2 End Boundary

The Catalogue record reaches the canonical Published state.

### 4.3 In Scope

- Resource identification.
- Bibliographic metadata preparation.
- Resource type classification.
- Classification-system application.
- Metadata verification.
- Librarian/authorised review.
- Catalogue approval.
- Catalogue publication.
- Publication history/audit.
- Downstream availability of the published catalogue record.

### 4.4 Out of Scope

- Physical copy creation.
- Accession/barcode assignment.
- Circulation.
- Acquisition procurement execution.
- Digital repository management.
- Student/personnel lifecycle.
- Financial accounting.
- Technical search/indexing implementation.

## 5. Ownership and Participating Domains

Library Management owns the Catalogue aggregate and its lifecycle.

Section E provides librarian/personnel context where operational responsibility is required. H may consume published catalogue information for governed communication. Neither domain owns the catalogue record.

Physical copies remain separately owned by the G05 Library Copy aggregate.

## 6. Business Actors, Roles and Responsibilities

### Librarian

Prepares, classifies, verifies and approves catalogue information according to configured library authority.

### Library Assistant

May support catalogue preparation and operational data verification.

### Library Administrator / Configured Authority

May perform approval responsibilities where required.

### Section E

Provides authoritative personnel context for librarians/library staff.

### H / Downstream Communication

May consume published library information where relevant; it does not own the catalogue.

## 7. Preconditions and Entry Criteria

1. A library resource has been identified.
2. Sufficient bibliographic information is available to prepare the catalogue record.
3. A configured resource type is available.
4. An applicable classification system is available.
5. Required review/approval authority is established.

## 8. Trigger

A library resource is identified for catalogue registration.

## 9. Main Business Flow

### Phase A — Identify and Prepare

1. Identify the intellectual library resource.
2. Establish the bibliographic information required for the catalogue record.
3. Record applicable metadata such as title, author, publisher, edition, ISBN, subject, language, keywords, classification and resource type.
4. Determine the applicable resource type.
5. Apply the configured classification system.
6. Validate the completeness and consistency of the catalogue information.

### Phase B — Verify and Approve

7. Submit the prepared catalogue record for operational review.
8. Librarian/authorised staff verify the bibliographic information and classification.
9. Resolve any identified data-quality issue.
10. Complete required approval.

### Phase C — Publish

11. Move the Catalogue aggregate through its authoritative lifecycle to Published.
12. Publish the canonical catalogue event `CatalogueCreated`.
13. Make the published catalogue record available to downstream library operations.
14. Preserve catalogue history and audit information.

The scenario ends here. Physical-copy creation, accessioning and circulation are subsequent workflows.

## 10. Alternate Business Flows

### 10.1 Incomplete Metadata

If required bibliographic information is insufficient, the record remains in the appropriate pre-publication condition until the information is completed. No new lifecycle state is invented.

### 10.2 Classification Requires Correction

The record is returned to catalogue preparation/review and corrected before publication.

### 10.3 Approval Rejected

The catalogue is not Published. The record returns to the appropriate review/preparation outcome.

### 10.4 Multiple Languages

G05 explicitly permits catalogue information to support multiple languages. The catalogue record may contain configured multilingual information without creating separate catalogue ownership.

### 10.5 Institution-Specific Metadata

Institutions may configure additional metadata. Additional fields do not alter the canonical Catalogue ownership or lifecycle.

### 10.6 Resource Type Is Institution-Specific

Resource types remain configurable. The scenario does not restrict the catalogue to the examples listed by L2.

## 11. Failure, Interruption and Business Recovery

### 11.1 Bibliographic Information Cannot Be Verified

Publication does not proceed until the information can be appropriately reviewed.

### 11.2 Classification Cannot Be Established

The record remains under catalogue preparation/review until a valid configured classification is established.

### 11.3 Approval Is Rejected

The record does not become Published and is returned for correction/review.

### 11.4 Resource Is Later Updated

A published catalogue may subsequently be modified through the Catalogue Update workflow. This scenario does not redefine that lifecycle.

### 11.5 Physical Copy Is Not Yet Available

Publication does not imply that a physical copy exists. Physical-copy registration is a separate workflow beginning with an available Catalogue record.

No technical retry, queue, rollback or deployment mechanics are introduced.

## 12. Business Rules and Decision Constraints

- Catalogue records represent intellectual works.
- Physical copies are managed separately.
- Typical catalogue information includes title, author, publisher, edition, ISBN, subject, language, keywords, classification and resource type.
- Institutions may configure additional metadata.
- Resource types remain configurable.
- Classification systems may include Dewey Decimal Classification, Library of Congress Classification, or institution-specific classification.
- Language configuration is institution configurable.
- Catalogue lifecycle is `Draft → Verified → Published → Archived`.
- Historical catalogue information is not to be deleted.
- Library Management owns catalogue operations.
- Personnel ownership remains E-owned.
- Downstream communication remains outside G05 catalogue ownership.

No mandatory metadata field beyond what the source supports, no specific classification system, and no technical search/indexing mechanism is invented.

## 13. State and Lifecycle Impact

The authoritative Catalogue lifecycle is:

`Draft → Verified → Published → Archived`

This scenario covers:

`Draft → Verified → Published`

The scenario does not invent:
- `Pending Approval`;
- `Rejected`;
- `Indexed`;
- `Searchable`;
- `CopyAvailable`.

Those are either business conditions within the workflow or belong to other aggregates.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

The G05 Catalogue Events catalogue defines:

- `CatalogueCreated` — Catalogue published.
- `CatalogueUpdated` — Catalogue modified.
- `CatalogueArchived` — Catalogue archived.

The publication outcome uses:

**`CatalogueCreated` — published by Library Management.**

The event name is retained exactly as defined by the locked L2, even though its description refers to publication.

### 14.2 Synchronous Business Collaboration / Hand-offs

- E supplies authoritative librarian/personnel context where required.
- Downstream library operations consume the published Catalogue record.
- Physical-copy management begins separately from a published/available catalogue resource.
- H may consume relevant published information.

No ownership transfers and no technical contract is invented.

## 15. Business Authorization, Approval and Audit Obligations

The G05 authorization model identifies Librarian as the typical authority for creating catalogue records and states that authorization policies remain configurable.

Catalogue creation/publication therefore follows configured library authority.

The catalogue preparation, classification, verification, approval and publication history must remain auditable. The G05 architecture requires immutable operational audit history and audited lifecycle transitions.

No technical access control/IAM implementation is described.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Resource identified.
- Bibliographic information prepared.
- Resource type established.
- Classification established.
- Record verified.
- Required approval completed.
- Catalogue reaches Published.
- `CatalogueCreated` published.
- Published record is available to subsequent library operations.
- Catalogue history preserved.

### Postconditions

- A canonical Published Catalogue record exists.
- Physical copies may subsequently reference the catalogue record.
- The catalogue remains independent of physical-copy lifecycle.
- Historical catalogue information remains traceable.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

No material ownership, lifecycle or publisher conflict identified.

The L2 event name `CatalogueCreated` is explicitly retained even though its description is “Catalogue published”; this is treated as the canonical upstream terminology and is not renamed.

### Assumptions

- Exact metadata completeness criteria remain configuration driven.
- Exact approval requirements remain configurable.
- Publication does not imply physical-copy availability.

### Deferred Questions

None material to the frozen scenario outcome.
