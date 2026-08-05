# L2-J02 — Vendor & Supplier Management Workflows

## Document Metadata

| Attribute | Value |
|---|---|
| Document ID | L2-J02 |
| Document Name | Vendor & Supplier Management Workflows |
| Section | J - Enterprise Resource Management |
| Domain | J02 - Vendor & Supplier Management |
| Architecture Level | Level 2 - Domain Workflow Architecture |
| Parent Architecture | L2-J00 - Enterprise Resource Management Architecture & Governance |
| Document Owner | Enterprise Resource Management — Vendor & Supplier |
| Primary Aggregate | Supplier |
| Canonical Owner | Vendor & Supplier Management |
| Status | Draft |
| Version | 1.1 |

---

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose |
| Scope | §3 Scope |
| Actors | — not present — |
| Entities | — not present — |
| Business Rules | Rules / Invariants |
| Workflow | Workflows |
| State Machine | States |
| Contracts | — not present — |
| Events | §13 Events |
| Dependencies | — not present — |
| Locked Decisions | — not present — |

> **Gap:** 5 required section(s) have no counterpart in this document: Actors, Entities, Contracts, Dependencies, Locked Decisions. Recorded, not authored.

---

# 1. Purpose

J02 defines the canonical architecture for supplier identity, supplier profile, supplier verification, supplier qualification, supplier eligibility, supplier sensitive-data governance, supplier restrictions, supplier suspension, supplier blacklisting, supplier reactivation, supplier closure, and supplier portal access boundaries within Section J.

J02 establishes one trusted Supplier Master used by procurement, finance, inventory, asset, campus, reporting, and other authorised SutraERP domains.

---

# 2. Domain Mission

J02 maintains a single, trustworthy, auditable, configurable, and institution-scoped Supplier Master without assuming ownership of procurement decisions, purchase orders, goods receipt, inventory, assets, accounting, or payments.

J02 answers:

- Who is this supplier?
- Is this supplier already known to the institution?
- Is the supplier verified?
- Is the supplier approved and active?
- For which categories and institutional scopes is the supplier qualified?
- What sensitive supplier information is governed?
- Is the supplier restricted, suspended, blacklisted, inactive, or closed?
- What supplier governance evidence exists?

---

# 3. Scope

J02 owns:

- Supplier registration.
- Supplier identity resolution.
- Duplicate supplier detection and governed merge.
- Supplier onboarding.
- Supplier verification.
- Supplier approval and activation.
- Supplier classification and categories.
- Supplier qualification and eligibility.
- Supplier documents and evidence references.
- Supplier tax and regulatory profile required for supplier identity and procurement use.
- Supplier banking profile required for authorised downstream Finance use.
- Supplier contacts and addresses.
- Supplier sensitive-data change workflow.
- Supplier performance evidence.
- Supplier issue and incident history.
- Supplier restriction, suspension, blacklisting, reactivation, inactivation, and closure.
- External/government procurement supplier mapping.
- Supplier portal business relationship to supplier identities.

---

# 4. Explicit Non-Ownership

| Capability | Canonical Owner |
|---|---|
| Procurement planning and sourcing | J01 - Procurement Planning & Sourcing |
| Purchase requisition and approval | J03 - Purchase Requisition & Approval |
| Purchase order | J04 - Purchase Order Management |
| Goods receipt and inspection | J05 - Goods Receipt, Inspection & Acceptance |
| Item master, warehouse, stock ledger | J06 - Inventory & Warehouse Management |
| Stock movement and distribution | J07 - Stock Operations & Distribution |
| Operational asset register | J08 - Asset Lifecycle Management |
| Physical verification and audit conclusion | J09 - Physical Verification, Audit & Reconciliation |
| Disposal/write-off case | J10 - Disposal, Scrap & Write-Off |
| Vendor invoice, tax accounting, payment, ledger posting | Section F - Finance |
| Authentication | L2-A03 - Authentication |
| Authorization policy, SoD, delegation | L2-A04 - Authorization and Policy |
| Workflow routing and human tasks | L2-A05 - Workflow Engine |
| Event transport and replay | L2-A06 - Event Infrastructure |
| Enterprise audit persistence and retention governance | L2-A07 - Audit Infrastructure |

Supplier approval does not mean supplier selection, purchase authorization, payment authorization, or tax-accounting approval.

---

# 5. Primary Aggregates

| Aggregate | Responsibility |
|---|---|
| Supplier | Canonical supplier identity, lifecycle, status, and profile references |
| SupplierVerification | Verification checks, evidence references, reviewer decisions, and validity |
| SupplierQualification | Category, scope, eligibility, and expiry information |
| SupplierSensitiveChange | Controlled changes to legal, tax, banking, contact, and other sensitive profile data |
| SupplierGovernanceCase | Restriction, suspension, blacklisting, reactivation, inactivation, closure, and issue history |
| SupplierMergeRecord | Duplicate confirmation, surviving supplier, historical mapping, and merge evidence |

---

# 6. Supplier Lifecycle

```text
Prospective / Registered
        |
        v
Submitted for Onboarding
        |
        v
Verification Pending
        |
        +--> Returned / Rejected
        |
        v
Approved
        |
        v
Active
        |
        +--> Restricted
        +--> Suspended
        +--> Blacklisted
        +--> Inactive
        |
        v
Closed / Archived
```

Closure shall not delete supplier history.

---

# 7. Supplier Identity and Duplicate Governance

One real-world supplier shall map to one canonical Supplier Identifier within an institution unless institutional governance explicitly permits scoped separation.

Duplicate detection may use legal name, tax identifiers, banking profile, contact data, external references, category, and historical supplier codes.

Duplicate detection is advisory until confirmed by an authorised governance process.

A confirmed duplicate shall not destructively delete historical supplier evidence. Merge shall preserve legacy identifiers, historical transaction references, and the surviving Supplier Identifier.

---

# 8. Supplier Sensitive Data

Sensitive supplier data may include:

- Legal identity information.
- PAN, GST, tax, or regulatory identifiers where applicable.
- Banking profile and account information.
- Authorised contacts and addresses.
- Supplier documents and evidence.
- Internal performance evidence.
- Suspension, restriction, or blacklisting evidence.
- External procurement mappings.

Sensitive fields shall support classification, masking where appropriate, purpose-bound visibility, maker-checker approval, effective dating, previous-value history, and audit evidence.

Banking profile approval by J02 does not authorize payment. Finance and Treasury must apply their own downstream payment controls.

---

# 9. Privacy, Confidentiality, and Data Protection

J02 owns supplier-domain privacy semantics for supplier information under Section J.

J02 shall support:

- Purpose limitation for supplier onboarding, procurement eligibility, finance consumption, compliance evidence, and historical traceability.
- Data minimization through stable references instead of copying externally owned person, institution, finance, or document data unnecessarily.
- Role and policy restricted access to banking, tax, performance, restriction, suspension, and blacklisting information.
- Supplier-portal visibility limited to information the supplier representative is authorised to see or maintain.
- Historical reconstruction without unnecessary exposure of obsolete sensitive details.
- Retention and archival governed by enterprise and institutional policy.
- Deletion only through enterprise-governed retention processes where legally and institutionally permitted.

J02 shall not expose private internal supplier performance history to supplier representatives by default.

---

# 10. Authorization and Segregation of Duties

J02 defines supplier-management authorization semantics while L2-A04 owns authorization policy evaluation.

Authorization may consider:

- Tenant and institution.
- Campus and organisational scope.
- Supplier category.
- Supplier lifecycle state.
- Sensitive-data classification.
- Supplier portal representative relationship.
- Verification authority.
- Approval authority.
- Governance-case role.
- Delegated authority.

Representative SoD rules include:

```text
Supplier Creator != Supplier Approver
Bank Profile Requester != Bank Profile Approver
Supplier Representative != Institutional Verifier
Performance Evidence Recorder != Blacklisting Approver where configured
Suspension Proposer != Reactivation Approver where configured
```

Expired delegation shall not remain valid.

---

# 11. Supplier Portal Trust Boundary

Supplier portal authentication belongs to L2-A03 Authentication.

Supplier portal authorization belongs to L2-A04 Authorization & Policy.

J02 owns only the business relationship between an authenticated supplier user and the Supplier identities they may represent.

Supplier representatives shall not directly:

- Approve their own supplier registration.
- Verify their own evidence.
- Qualify themselves.
- Remove institutional restrictions.
- Remove suspension or blacklisting.
- Approve their own banking changes.
- Alter internal performance history.
- Alter audit history.

---

# 12. Audit and Traceability

J02 produces domain audit evidence for:

- Supplier registration, submission, return, rejection, approval, activation, inactivation, closure, and reactivation.
- Identity changes and duplicate merge decisions.
- Verification checks and evidence decisions.
- Qualification and eligibility changes.
- Document upload, renewal, expiry, and verification state.
- Tax, regulatory, contact, address, and banking profile changes.
- Sensitive-data change request, approval, rejection, and application.
- Performance evidence and supplier incident records.
- Restriction, suspension, blacklisting, removal, and governance-case decisions.
- Supplier portal representative mapping and self-service requests.
- External supplier mapping and migration decisions.

Audit records shall preserve actor, tenant, institution, timestamp, aggregate, previous state where applicable, resulting state, reason, authority, correlation reference, causation reference, and evidence references.

L2-A07 owns enterprise audit persistence, retention, integrity, evidence packaging, restoration, and audit access governance.

---

# 13. Events

J02 owns supplier business event meaning and timing.

J02 may publish:

- SupplierRegistered
- SupplierSubmittedForVerification
- SupplierVerificationCompleted
- SupplierApproved
- SupplierActivated
- SupplierRejected
- SupplierProfileChanged
- SupplierSensitiveChangeRequested
- SupplierSensitiveChangeApproved
- SupplierBankProfileApproved
- SupplierQualificationGranted
- SupplierQualificationExpired
- SupplierRestricted
- SupplierSuspended
- SupplierBlacklisted
- SupplierReactivated
- SupplierInactivated
- SupplierClosed
- SupplierMergeCompleted
- ExternalSupplierMapped

Events represent completed business facts, not commands.

L2-A06 owns event transport, routing, retry, replay, dead-letter handling, event registry governance, and subscription infrastructure.

---

# 14. Configuration Governance

J02 configuration shall be tenant and institution scoped unless explicitly defined as enterprise-global.

Configurable elements include:

- Supplier categories.
- Required onboarding stages.
- Required document classes.
- Verification rules.
- Approval authorities.
- Sensitive-field classifications.
- Banking profile controls.
- Qualification scopes.
- Restriction, suspension, and blacklisting rules.
- Portal self-service permissions.
- Retention and archival rules where policy permits domain-level configuration.

Material configuration shall be effective-dated, versioned, approved, and historically traceable.

---

# 15. Integration Architecture

J02 integrates through contracts, events, and read models.

| Domain | Interaction | Ownership Boundary |
|---|---|---|
| J01 - Procurement Planning & Sourcing | Supplier eligibility and sourcing participation | J01 owns sourcing decision |
| J03 - Purchase Requisition & Approval | Suggested supplier references where policy permits | J03 owns requisition |
| J04 - Purchase Order Management | Approved supplier identity and eligibility | J04 owns purchase order |
| J05 - Goods Receipt | Supplier references for receipt evidence | J05 owns receipt facts |
| Finance | Supplier tax/profile/banking information through authorised contracts | Finance owns accounting and payment consequences |
| H07/H08/H09 reporting services | Governed supplier read models and evidence mapping | Reporting does not own supplier truth |
| A03/A04/A05/A06/A07 | Platform trust services | Platform owns mechanisms |

J02 shall not directly write into another domain's persistence.

Other domains shall not maintain an independent authoritative Supplier Master.

---

# 16. Operational Governance and Recovery

J02 shall support idempotent command handling for registration submission, verification completion, sensitive-change approval, bank-profile approval, supplier activation, restriction, suspension, blacklisting, reactivation, closure, merge, and external mapping.

Technical failure shall not cause loss of confirmed supplier identity, approval, sensitive-change, banking, restriction, suspension, blacklisting, or merge evidence.

External procurement supplier mappings shall preserve external provenance without creating a parallel Supplier Master.

---

# 17. Compliance Readiness

J02 supports architectural readiness for privacy-by-design, security-by-design, least privilege, immutable evidence, supplier due diligence, procurement transparency, financial fraud prevention, and institutional governance.

J02 does not determine legal compliance obligations by itself. Compliance interpretation belongs to the relevant institutional compliance capability, while J02 preserves supplier evidence and controls needed for compliance assessment.

---

# 18. Reporting and Governance Views

J02 may expose governed read models for:

- Supplier register.
- Supplier verification status.
- Supplier qualification and eligibility.
- Sensitive-change queue.
- Expiring documents.
- Bank-profile change history.
- Restricted, suspended, and blacklisted suppliers.
- Supplier performance evidence.
- Supplier portal requests.
- Duplicate resolution and merge history.

Read models shall not become authoritative supplier owners.

---

# 19. Completion Criteria

J02 is complete when:

- Supplier Master ownership is explicit and exclusive.
- Supplier identity, onboarding, verification, approval, qualification, sensitive changes, banking profile, restrictions, suspension, blacklisting, reactivation, closure, duplicate resolution, external mapping, and portal boundaries are defined.
- Authorization, SoD, privacy, tenant isolation, audit evidence, events, configuration, integration, operational recovery, reporting, and retention are explicit.
- Finance consumption of supplier banking/profile information does not transfer Supplier Master ownership.
- Supplier portal users cannot approve or alter their own governed supplier state.
- No downstream domain maintains a competing Supplier Master.

---

# 20. Final Architecture Statement

J02 is the canonical owner of supplier identity and supplier governance within SutraERP.

It preserves trusted supplier information, sensitive supplier data controls, supplier governance evidence, and supplier lifecycle history while procurement, finance, inventory, assets, audit, reporting, and platform capabilities retain their separate canonical responsibilities.

---

## Workflows

J02's core workflows are enumerated in Section 3 (Scope): supplier registration, identity resolution and duplicate merge, onboarding, verification, approval and activation, classification and qualification, sensitive-data change, performance and incident recording, and restriction/suspension/blacklisting/reactivation/inactivation/closure. Step-level detail for these workflows is expressed through the lifecycle transitions in Section 6 and the governance rules in Sections 7-12.

## States

Supplier lifecycle states (Prospective/Registered, Submitted for Onboarding, Verification Pending, Returned/Rejected, Approved, Active, Restricted, Suspended, Blacklisted, Inactive, Closed/Archived) are defined in the Supplier Lifecycle diagram (Section 6).

## Rules / Invariants

Domain rules and invariants are stated throughout the document rather than consolidated separately: identity/duplicate governance rules (Section 7), sensitive-data controls (Section 8), authorization and segregation-of-duties rules (Section 10), portal self-service restrictions (Section 11), and operational/idempotency guarantees (Section 16).
