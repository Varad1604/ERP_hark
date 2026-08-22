# Catalogue Update to Effective Resource Information

**Scenario ID:** L3-G-128  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-128 |
| Canonical Title | Catalogue Update to Effective Resource Information |
| Primary Owner | Library Management |
| Participating Domains | G |
| Scenario Type | Simple |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the complete business outcome in which an existing Library Catalogue record is identified as requiring correction or update, the revised resource information is prepared and reviewed, and the updated information becomes effective within Library Management.

The scenario is deliberately limited to catalogue information. A Catalogue represents an intellectual work; physical Library Copies remain a separate aggregate and are not changed merely because catalogue information changes.

The L2 architecture defines Catalogue Management as governing bibliographic information and explicitly separates catalogue records from physical copies. The catalogue information may include title, author, publisher, edition, ISBN, subject, language, keywords, classification and resource type, with additional metadata remaining institution-configurable.

## 3. Business Outcome

The authoritative catalogue record contains the corrected/effective resource information, the update is incorporated into the Catalogue lifecycle without changing ownership, and the updated catalogue information is available to subsequent library operations.

The historical catalogue information and update history remain preserved.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An existing Catalogue record is identified as requiring an information update.

### 4.2 End Boundary

The corrected catalogue information has been accepted as the effective information for the Catalogue record and the update history has been preserved.

### 4.3 In Scope

- Identification of the Catalogue record.
- Identification of the information requiring update.
- Preparation of revised bibliographic information.
- Validation of the revised information.
- Classification/resource-type review where affected.
- Authorised catalogue review.
- Effective update of the Catalogue record.
- Audit/history.
- Publication of the canonical catalogue update event.

### 4.4 Out of Scope

- Creation of a new intellectual work.
- Physical-copy registration or condition management.
- Accession number or barcode changes.
- Circulation transactions.
- Acquisition/procurement execution.
- Student/personnel lifecycle.
- Finance/accounting.
- Technical search indexing, APIs, databases, UI or infrastructure.

## 5. Ownership and Participating Domains

Library Management is the canonical owner of the Catalogue aggregate and its bibliographic information.

No participating external domain is defined for this scenario. Personnel who perform the work remain governed by E's personnel ownership, but E does not participate in the frozen scenario identity.

The separation between Catalogue and Library Copy ownership is mandatory: every Library Copy references one Catalogue record, but copy operations are separate.

## 6. Business Actors, Roles and Responsibilities

### Librarian

Prepares, reviews and authorises catalogue information changes according to configured library authority.

### Library Assistant

May support information preparation and verification where permitted by configured operational authority.

The exact authority remains configurable; this scenario does not create new roles.

## 7. Preconditions and Entry Criteria

1. A Catalogue record exists.
2. The record can be identified unambiguously.
3. A business reason for the update is known.
4. The revised information can be prepared or obtained.
5. The proposed update does not violate a locked business invariant.
6. Required library review/authority is available.

## 8. Trigger

A Catalogue record is identified as requiring updated resource information.

## 9. Main Business Flow

### Phase A — Identify the Update

1. Identify the existing Catalogue record.
2. Determine which catalogue information is no longer effective or requires correction.
3. Establish the intended revised information.
4. Determine whether the update affects bibliographic fields, classification, resource type, language information, or institution-configured metadata.

### Phase B — Prepare and Validate

5. Prepare the revised catalogue information.
6. Validate the revised information for completeness and consistency.
7. Reassess classification when classification information changes.
8. Reassess resource type when resource-type information changes.
9. Confirm that the update continues to represent the same intellectual work rather than creating a separate catalogue identity.

### Phase C — Review and Make Effective

10. Submit the update for configured library review.
11. Resolve any identified information-quality issue.
12. Complete required authorisation.
13. Apply the revised information to the Catalogue record.
14. Preserve the prior historical information and update history.
15. Publish `CatalogueUpdated`.

### Phase D — Complete

16. Treat the revised information as the effective catalogue information.
17. Make the updated Catalogue record available to subsequent library operations.
18. End the scenario.

The scenario does not create or modify physical copies.

## 10. Alternate Business Flows

### 10.1 Update Does Not Affect the Intellectual Work

The Catalogue record is updated in place and its identity remains unchanged.

### 10.2 Classification Requires Reassessment

Where classification changes, the revised classification is reviewed before the update becomes effective. The classification system remains institution-configurable.

### 10.3 Resource Type Requires Reassessment

The revised resource type is validated against the institution's configured resource types. No fixed list beyond the L2 examples is imposed.

### 10.4 Multilingual Information Is Updated

Catalogue information may support multiple languages, and language configuration remains institution configurable. The update changes the catalogue information without creating a new aggregate.

### 10.5 Update Is Rejected

If the revised information is not acceptable, the effective catalogue information remains unchanged and the update does not become authoritative.

### 10.6 Update Actually Represents a New Intellectual Work

If review determines that the requested change would create a different intellectual work rather than update the existing one, this scenario does not create the new catalogue record. The matter exits this scenario for the appropriate catalogue-creation workflow.

## 11. Failure, Interruption and Business Recovery

### 11.1 Revised Information Cannot Be Verified

The update remains non-effective until the information can be appropriately verified.

### 11.2 Required Review Is Not Completed

The existing effective catalogue information remains authoritative.

### 11.3 Proposed Update Violates a Business Invariant

The update is rejected. G05 states that violation of business invariants rejects the transaction.

### 11.4 Historical Information Is Required

The previous catalogue information is retained because historical catalogue information shall never be deleted.

No technical retry, queue, rollback or infrastructure behaviour is defined.

## 12. Business Rules and Decision Constraints

- Catalogue records represent intellectual works.
- Physical copies are managed separately.
- Catalogue information may include title, author, publisher, edition, ISBN, subject, language, keywords, classification and resource type.
- Additional metadata may be institution-configured.
- Resource types remain configurable.
- Classification systems remain configurable.
- Multilingual catalogue information is supported.
- Historical catalogue information shall never be deleted.
- Every lifecycle transition shall be audited.
- Every Library Copy references exactly one Catalogue record.
- Library Management owns the catalogue information.

The scenario does not invent mandatory metadata fields beyond the L2-supported model.

## 13. State and Lifecycle Impact

The authoritative Catalogue lifecycle is:

`Draft → Verified → Published → Archived`

This scenario is an update to an existing Catalogue record. The L2 does not define a separate `Updated` lifecycle state.

The update therefore does not invent a new state. It results in revised effective information while preserving the existing Catalogue lifecycle semantics.

Where the update occurs on a published catalogue, the catalogue remains within the existing lifecycle and its revised information becomes effective through the catalogue update workflow.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Event

The exact canonical event is:

**`CatalogueUpdated` — Catalogue modified.**

The event is published by Library Management.

No `ResourceInformationChanged` or `CatalogueModified` event is invented.

### 14.2 Business Collaboration

This is a G-only canonical scenario. Any personnel performing the work remain subject to the enterprise personnel/authorization architecture, but no external domain is assigned ownership or participation in the frozen scenario.

## 15. Business Authorization, Approval and Audit Obligations

The G05 authorization model identifies Librarian as a typical authority for catalogue creation and states that authorization policies remain configurable.

Accordingly:

- the update must be performed by an appropriately authorised library operator;
- configured review/approval requirements must be respected;
- the change history must remain auditable;
- historical catalogue information must remain preserved.

No technical access control/IAM implementation is defined at L3.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Existing Catalogue identified.
- Required information update identified.
- Revised information prepared.
- Relevant metadata/classification/resource-type implications reviewed.
- Required library review completed.
- Revised information accepted.
- `CatalogueUpdated` published.
- Historical catalogue information preserved.

### Postconditions

- The Catalogue contains the effective revised resource information.
- The Catalogue remains the canonical representation of the intellectual work.
- Existing Library Copies continue to reference the Catalogue record.
- No physical-copy lifecycle is changed solely by this scenario.
- Historical catalogue information remains available.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

No material ownership, lifecycle or event-publisher conflict identified.

### Assumptions

- Exact catalogue update approval thresholds remain configurable.
- The update is intended to preserve the identity of the existing intellectual work.

### Deferred Questions

None material to the frozen scenario outcome.
