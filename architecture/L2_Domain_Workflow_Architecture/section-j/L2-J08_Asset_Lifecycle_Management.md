# L2-J08 — Asset Lifecycle Management

## Document Metadata

| Attribute | Value |
|---|---|
| Document ID | L2-J08 |
| Document Name | Asset Lifecycle Management |
| Section | J - Enterprise Resource Management |
| Domain | J08 - Asset Lifecycle Management |
| Architecture Level | Level 2 |
| Document Type | Domain Workflow Architecture |
| Parent Architecture | L2-J00 - Enterprise Resource Management Architecture & Governance |
| Primary Aggregate | Asset |
| Canonical Owner | Operational Asset Lifecycle Management |
| Document Owner | Chetan |
| Status | Trust Certification Ready |
| Version | 1.1 |

---

# 1. Purpose

J08 defines the canonical operational asset lifecycle architecture for SutraERP.

J08 owns the Operational Asset Register and the business lifecycle of institution-managed assets from registration through custody, assignment, movement, condition tracking, verification participation, retirement coordination, and archival.

Finance owns financial asset accounting, book value, depreciation accounting, capitalization accounting, and disposal accounting consequences.

---

# 2. Scope

J08 owns:

- Operational Asset Register
- Asset Identity and Tagging
- Asset Classification
- Asset Registration
- Asset Custody
- Asset Assignment
- Asset Holder History
- Asset Location History
- Asset Condition History
- Warranty Reference
- AMC Reference
- Insurance Reference
- Asset Transfer
- Asset Return
- Asset Retirement Recommendation
- Asset Retirement State
- Asset Lifecycle Audit Metadata
- Asset Operational Read Models

---

# 3. Explicit Non-Ownership

| Capability | Canonical Owner |
|---|---|
| Supplier Master | J02 - Vendor & Supplier Management |
| Purchase Order | J04 - Purchase Order Management |
| Goods Receipt and Inspection | J05 - Goods Receipt, Inspection & Acceptance |
| Item Master, Warehouse, Stock Ledger | J06 - Inventory & Warehouse Management |
| Stock Movement Execution | J07 - Stock Operations & Distribution |
| Physical Verification / Inventory Audit Conclusion | J09 - Physical Verification, Audit & Reconciliation |
| Disposal / Scrap / Write-Off Business Case | J10 - Disposal, Scrap & Write-Off |
| Financial Asset Accounting | L2-F12 - Asset Financial Accounting |
| General Ledger and Depreciation Accounting | Section F - Finance |
| Authentication | L2-A03 - Authentication |
| Authorization Policy | L2-A04 - Authorization and Policy |
| Workflow Routing | L2-A05 - Workflow Engine |
| Event Transport | L2-A06 - Event Infrastructure |
| Enterprise Audit Persistence | L2-A07 - Audit Infrastructure |

J08 shall not directly modify Finance ledgers, depreciation records, inventory stock balances, or disposal accounting records.

---

# 4. Primary Aggregates

| Aggregate | Responsibility |
|---|---|
| Asset | Governs operational asset identity, lifecycle state, custody, assignment, condition, and references |
| AssetAssignment | Governs assignment and return of an asset to a person, department, location, or operational unit |
| AssetTransfer | Governs operational transfer of an asset between authorised custodial contexts |
| AssetLifecycleEvent | Preserves immutable operational history for material lifecycle changes |

---

# 5. Canonical Lifecycle

```text
Asset Candidate
        |
        v
Asset Registered
        |
        v
Available / In Custody
        |
        +--> Assigned
        |       |
        |       v
        |    Returned
        |
        +--> Transferred
        |
        +--> Under Repair / Maintenance Reference
        |
        +--> Verification Pending / Verified
        |
        v
Retirement Recommended
        |
        v
Retired / Disposal Coordinated
        |
        v
Archived
```

---

# 6. Business Rules

1. Every managed asset shall have one stable Asset Identifier within its tenant and institution.
2. Asset tag changes shall not change asset identity.
3. Custody, assignment, location, and condition changes shall preserve historical evidence.
4. Verification findings from J09 shall not silently overwrite J08 asset state.
5. Disposal/write-off execution shall occur through J10 where applicable.
6. Finance asset accounting status shall not determine operational custody by itself.
7. Asset lifecycle corrections shall use governed amendments or supplemental evidence rather than destructive editing.
8. Cross-institution asset transfer requires explicit governed authorization.

---

# 7. Authorization & Segregation of Duties

J08 defines asset-management authorization semantics while L2-A04 owns authorization policy evaluation.

Authorization may consider:

- Tenant and institution.
- Campus, department, location, and custodian.
- Asset category and controlled-asset classification.
- Lifecycle state.
- Assignment scope.
- Transfer source and destination.
- Value band where institutionally configured.
- Delegated authority.

Representative SoD rules include:

```text
Asset Registrar != Asset Approver where configured
Asset Custodian != Independent Verifier
Transfer Requester != Transfer Approver
Retirement Recommender != Final Retirement Approver
```

---

# 8. Privacy and Sensitive Asset Controls

Asset records may reference employees, students, departments, rooms, sensitive equipment, or controlled institutional resources.

J08 shall minimize embedded externally owned personal or institutional data and shall reference canonical identifiers from owning domains wherever possible.

Controlled or sensitive assets shall support stricter visibility, custody, transfer, verification, and retirement policies without creating a second asset register.

---

# 9. Audit & Evidence

J08 produces domain evidence for:

- Asset registration.
- Tag assignment or tag replacement.
- Custody assignment.
- Asset transfer.
- Location change.
- Condition update.
- Warranty or AMC reference change.
- Verification status update.
- Retirement recommendation.
- Lifecycle closure.

L2-A07 remains the canonical owner of enterprise audit persistence, retention governance, access governance, and evidence integrity.

---

# 10. Tenant and Institution Isolation

Every J08 aggregate, command, event, read model, audit reference, and integration message shall preserve tenant and institution context.

An Asset Identifier alone shall never authorize access.

---

# 11. Domain Events

J08 publishes business facts such as:

- AssetRegistered
- AssetTagAssigned
- AssetAssigned
- AssetReturned
- AssetTransferred
- AssetConditionChanged
- AssetVerificationStatusUpdated
- AssetRetirementRecommended
- AssetRetired
- AssetArchived

Event transport, retry, replay, routing, subscription, and registry governance remain owned by L2-A06.

---

# 12. Integration

J08 integrates with:

| Domain | Interaction | Ownership Boundary |
|---|---|---|
| J05 - Goods Receipt, Inspection & Acceptance | Accepted goods may qualify for asset registration | J05 owns receipt facts |
| J06 - Inventory & Warehouse Management | Asset may originate from inventory-controlled items | J06 owns inventory truth |
| J07 - Stock Operations & Distribution | Asset-linked stock movement may inform custody transfer | J07 owns stock movement facts |
| J09 - Physical Verification, Audit & Reconciliation | Verification consumes and produces asset evidence | J09 owns audit conclusions |
| J10 - Disposal, Scrap & Write-Off | Retired assets may enter disposal/write-off case | J10 owns disposal case and execution evidence |
| Finance F12 | Consumes asset facts for accounting | Finance owns financial asset accounting |
| HR / Student / Organisation | Provides canonical person, department, and context references | Owning domains retain master-data ownership |

---

# 13. Resilience and Recovery

J08 commands that may be retried shall support idempotency using command identifier, idempotency key, aggregate identifier, correlation identifier, and expected version where applicable.

Technical failure shall not cause loss of confirmed custody, assignment, transfer, or retirement evidence.

---

# 14. Retention and Archival

Operational asset history shall remain reconstructable for institutional governance, financial reconciliation, audit, accreditation, and legal evidence.

Archived assets shall remain tenant and institution scoped and accessible only according to authorization and retention policy.

Physical deletion shall occur only through the enterprise-governed retention process where policy permits.

---

# 15. Completion Criteria

J08 is complete when:

- Operational Asset Register ownership is explicit.
- Asset identity, custody, assignment, transfer, condition, retirement, and archival lifecycles are defined.
- J08 does not own procurement, goods receipt, inventory truth, stock movement execution, disposal case execution, or financial accounting.
- Authorization, SoD, tenant isolation, audit evidence, privacy controls, events, integration contracts, resilience, and retention are explicit.
- No non-Section-J content exists in this document.

---

# 16. Final Architecture Statement

J08 is the canonical owner of operational asset lifecycle truth.

It preserves asset identity, custody, assignment, movement, condition, retirement, and operational evidence while Finance owns financial accounting and J10 owns disposal/write-off execution.

---

## Workflows

Asset lifecycle workflow steps are captured in Section 2 (Scope): asset registration, identity and tagging, classification, custody, assignment, holder/location/condition history, warranty/AMC/insurance reference, transfer, return, and retirement recommendation, sequenced through the Canonical Lifecycle in Section 5.

## States

This document does not define a discrete state table separate from the lifecycle flow in Section 5; asset progression is expressed through the stages Asset Candidate, Registered, Available/In Custody, Assigned, Returned, Transferred, Under Repair/Maintenance Reference, Verification Pending/Verified, Retirement Recommended, Retired/Disposal Coordinated, and Archived.
