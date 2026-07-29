````markdown
# L2-J02 — Vendor & Supplier Management Workflows

| Attribute | Value |
|---|---|
| Document Name | L2-J02 — Vendor & Supplier Management Workflows |
| Domain Code | J02 |
| Domain Name | Vendor & Supplier Management |
| Section | J — Enterprise Resource Management |
| Architecture Layer | Level 2 — Domain Workflow Architecture |
| Parent Architecture | L2-J00 — Enterprise Resource Management Architecture & Governance |
| Architecture Style | Domain-Driven Design (DDD), CQRS, Event-Driven Architecture (EDA), API-First Architecture |
| Canonical Owner | Vendor & Supplier Management Domain |
| Primary Aggregate | Supplier |
| Communication Pattern | Commands, Queries, Domain Events |
| Status | Architecture Draft |
| Version | 1.0 |
| Repository | SutraERP Canonical Enterprise Repository |
| Classification | Canonical Enterprise Architecture |

---

# Table of Contents

1. Purpose
2. Domain Mission
3. Architectural Position
4. Scope
5. Explicit Non-Ownership
6. Supplier Operating Model
7. Architectural Principles
8. Actors & Roles
9. Supplier Identity Architecture
10. Supplier Registration
11. Supplier Onboarding
12. Supplier Verification
13. Supplier Approval & Activation
14. Supplier Classification & Categories
15. Supplier Document Management
16. Supplier Tax & Regulatory Information
17. Supplier Banking Information
18. Supplier Contact & Address Management
19. Supplier Qualification & Eligibility
20. Campus & Organizational Eligibility
21. Supplier Profile Amendment
22. Sensitive Supplier Data Change
23. Supplier Performance History
24. Supplier Issue & Incident History
25. Supplier Suspension
26. Supplier Blocking & Restriction
27. Supplier Blacklisting
28. Supplier Reactivation
29. Supplier Inactivation & Closure
30. Duplicate Supplier Detection & Resolution
31. Supplier Merge & Identity Correction
32. External / Government Procurement Supplier Mapping
33. Existing Supplier Migration & Progressive Verification
34. Supplier Portal Architecture
35. Aggregate Catalogue
36. Command Catalogue
37. Query & Read Model Catalogue
38. Domain Event Catalogue
39. Consumed Event Catalogue
40. Business Rules & Invariants
41. Authorization & Segregation of Duties
42. Configuration Architecture
43. Privacy, Confidentiality & Sensitive Information
44. Audit & Traceability
45. Exception & Reconciliation Architecture
46. Notifications
47. Documents & Evidence
48. Cross-Domain Integration
49. Reporting & Governance Views
50. Future Extension Points
51. Architectural Constraints
52. Governance Decisions
53. Workflow Catalogue
54. Dependency Matrix
55. Completion Criteria

---

# 1. Purpose

## 1.1 Purpose Statement

J02 defines the canonical architecture for **Vendor & Supplier Management** within SutraERP Section J — Enterprise Resource Management.

The domain governs the institutional lifecycle of organizations and individuals that may provide:

- Goods
- Materials
- Equipment
- Services
- Professional Services
- Consultancy
- Maintenance
- Works
- Outsourced Services
- Technology Products and Services
- Other institutionally procured resources

J02 establishes the authoritative institutional Supplier identity used throughout SutraERP.

The canonical lifecycle is:

```text
Supplier Identified
        │
        ▼
Registration
        │
        ▼
Duplicate / Identity Check
        │
        ▼
Documentation
        │
        ▼
Verification
        │
        ▼
Qualification
        │
        ▼
Approval
        │
        ▼
Activation
        │
        ▼
Canonical Supplier
        │
   ┌────┼──────────────┐
   │    │              │
   ▼    ▼              ▼
  J01  J04          Finance
Sourcing PO       Financial Processing
        │
        ▼
Ongoing Supplier Governance
        │
        ├── Profile Maintenance
        ├── Document Renewal
        ├── Qualification Management
        ├── Eligibility Management
        ├── Private Performance History
        ├── Issue / Incident History
        ├── Restriction
        ├── Suspension
        ├── Blacklisting
        ├── Reactivation
        └── Inactivation / Closure
````

J02 shall support both simple supplier administration used by smaller colleges and highly controlled supplier-governance processes required by larger universities.

---

## 1.2 Primary Objective

The primary objective of J02 is to ensure that every supplier used across SutraERP resolves to a trusted canonical identity.

J02 shall answer:

```text
Who is this supplier?

Is this the same supplier already known to the institution?

What type of supplier is it?

What goods/services can it provide?

Has the supplier been verified?

Is the supplier approved?

For which categories is the supplier qualified?

Where is the supplier eligible?

Are required documents valid?

Is the supplier currently active?

Is the supplier restricted, suspended or blacklisted?

What institutional governance history exists for the supplier?
```

J02 shall answer these questions without taking ownership of procurement selection, purchase orders, inventory, assets, accounting or payments.

---

## 1.3 Institutional Applicability

The architecture shall support:

* Private Colleges
* Government Colleges
* Private Universities
* Public Universities
* Deemed Universities
* Autonomous Institutions
* Multi-Campus Universities
* Educational Trusts
* Educational Groups
* Research Institutions
* Institutions with Centralized Procurement
* Institutions with Decentralized Procurement
* Institutions using Government / External Procurement Systems
* Institutions heavily dependent on outsourced service providers

The architecture shall not assume that all institutions use the same supplier onboarding process.

---

## 1.4 Supplier Terminology

Within J02, the canonical enterprise concept is:

> **Supplier**

The term may represent entities commonly referred to institutionally as:

* Vendor
* Supplier
* Contractor
* Service Provider
* Consultant
* Agency
* Dealer
* Distributor
* Manufacturer
* Professional Service Provider

Institution-facing terminology may remain configurable.

The underlying architecture shall avoid maintaining separate Vendor Master, Contractor Master, Consultant Master and Service Provider Master implementations where the entities represent procurement counterparties.

---

# 2. Domain Mission

The mission of J02 is:

> **To maintain a single, trustworthy, configurable and auditable institutional Supplier Master that establishes supplier identity, verification, qualification, eligibility and governance without assuming ownership of procurement decisions, purchasing commitments, accounting transactions or payments.**

J02 exists to prevent fragmented supplier identities across institutional departments and ERP modules.

Without a canonical supplier domain, the following anti-pattern can emerge:

```text
Procurement Vendor
        │
        ├── Vendor #184

Finance Vendor
        │
        ├── Creditor #728

Campus Vendor
        │
        ├── Supplier #92

Maintenance Contractor
        │
        └── Agency #44
```

All four records may represent the same real-world organization.

J02 shall instead establish:

```text
              Canonical Supplier
                J02-SUP-00184
                      │
        ┌─────────────┼─────────────┐
        │             │             │
        ▼             ▼             ▼
       J01           J04         Finance
    Procurement   Purchase      Financial
     Context       Orders        Context
```

---

# 3. Architectural Position

## 3.1 Section J Position

J02 operates as the canonical supplier-management bounded context within Section J.

```text
                  SECTION J
        Enterprise Resource Management

                        │
                        ▼
                       J02
            Vendor & Supplier Management
                        │
          ┌─────────────┼─────────────┐
          │             │             │
          ▼             ▼             ▼
         J01           J04         Finance
     Procurement      Purchase      Financial
      Sourcing         Orders       Processing
```

J02 owns supplier identity and supplier governance.

J01 owns procurement sourcing.

J04 owns Purchase Orders.

Finance owns financial transactions.

---

## 3.2 Canonical Ownership

The fundamental ownership model is:

```text
                       J02
             Vendor & Supplier Management
                        │
              CANONICAL OWNER OF
                        │
        ┌───────────────┼───────────────┐
        │               │               │
        ▼               ▼               ▼
 Supplier Identity   Verification   Qualification
        │               │               │
        ├───────────────┼───────────────┤
        │               │               │
        ▼               ▼               ▼
     Profile        Eligibility      Governance
                        │
                        ▼
                Canonical SupplierId
                        │
       ┌────────────────┼────────────────┐
       ▼                ▼                ▼
      J01              J04            Finance
```

Other domains may maintain projections or references to Supplier information.

They shall not maintain competing canonical Supplier Masters.

---

## 3.3 Identity versus Transaction Ownership

J02 owns:

```text
Supplier
```

It does not own:

```text
Supplier + Quotation
Supplier + Purchase Order
Supplier + Goods Receipt
Supplier + Invoice
Supplier + Payment
```

Those transactional relationships belong to their respective domains.

---

## 3.4 Supplier Lifecycle versus Procurement Lifecycle

These lifecycles are related but distinct.

```text
SUPPLIER LIFECYCLE

Registration
    ↓
Verification
    ↓
Qualification
    ↓
Approval
    ↓
Active Supplier
```

versus:

```text
PROCUREMENT LIFECYCLE

Requirement
    ↓
Sourcing
    ↓
Supplier Response
    ↓
Evaluation
    ↓
Selection
    ↓
Purchase Order
```

J02 owns the first.

J01/J03/J04 collectively participate in the second according to their canonical boundaries.

---

# 4. Scope

## 4.1 Supplier Registration

J02 owns supplier registration through:

* Authorized Internal Users
* Supplier Self-Registration through a future Supplier Portal
* External Procurement Mapping
* Legacy Migration
* Administrative Import
* Approved Integration

Registration establishes a candidate/prospective Supplier identity.

Registration does not automatically establish approval.

---

## 4.2 Supplier Identity

J02 owns canonical supplier identity including applicable:

* Supplier ID
* Legal Name
* Trade Name
* Supplier Type
* Organization Type
* Registration References
* Tax Identity References
* Addresses
* Contacts
* External Supplier References
* Legacy Supplier References
* Supplier Lifecycle Status

---

## 4.3 Supplier Verification

J02 owns institutional verification state for applicable supplier information.

Verification may include:

* Identity Verification
* Legal Registration Verification
* Tax Information Verification
* Contact Verification
* Address Verification
* Banking Information Verification
* Document Verification
* Category Capability Verification
* Institution-Specific Verification

Verification depth shall remain configurable.

---

## 4.4 Supplier Approval

J02 owns the institutional decision that a Supplier is approved for applicable use.

Approval shall not imply:

* Procurement Selection
* Purchase Order Approval
* Payment Approval
* Guaranteed Business
* Guaranteed Tender Participation

---

## 4.5 Supplier Qualification

J02 owns supplier qualification and eligibility information.

Qualification may be defined according to:

* Supplier Category
* Goods Category
* Service Category
* Certification
* Licence
* Campus
* Organizational Scope
* Validity Period
* Institution Policy

---

## 4.6 Supplier Categories

A single supplier may operate in multiple categories.

Example:

```text
ABC Technologies Pvt. Ltd.
          │
          ├── Computers
          ├── Networking Equipment
          ├── Software
          ├── AMC Services
          └── IT Consultancy
```

Each category may independently have:

* Declared Capability
* Verification State
* Qualification State
* Eligibility State
* Validity Period

---

## 4.7 Supplier Documents

J02 owns supplier-document business requirements and document relationships.

Applicable evidence may include:

* Registration Documents
* PAN-related information where applicable
* GST-related information where applicable
* MSME-related evidence where applicable
* Bank Evidence
* Licences
* Certifications
* Insurance Evidence
* Manufacturer / Dealer Authorization
* Professional Registrations
* Institution-Specific Declarations

The enterprise document capability may own physical document storage.

---

## 4.8 Supplier Banking Profile

J02 owns the canonical approved supplier banking/profile information required by authorized downstream consumers.

Sensitive changes shall support controlled verification and approval.

Finance remains owner of:

* Accounts Payable
* Payment Processing
* Financial Settlement
* Accounting
* Payment Authorization

---

## 4.9 Supplier Performance History

J02 shall support **private internal supplier performance history**.

Performance evidence may originate from:

* Procurement Experience
* Delivery Experience
* Inspection Outcomes
* Goods Rejection
* Service Delivery
* Maintenance Experience
* Corrective Actions
* Operational Incidents

J02 shall not implement a generic public supplier rating system.

The following model is explicitly rejected:

```text
Supplier Rating: ★★★★☆
Supplier Score: 82/100
```

Instead:

```text
Supplier
   │
   ▼
Private Performance Evidence
   │
   ├── Delivery Issue
   ├── Quality Issue
   ├── Service Issue
   ├── Positive Performance Evidence
   ├── Corrective Action
   └── Resolution
```

Authorized personnel may use this evidence as procurement decision support.

---

## 4.10 Supplier Governance

J02 owns supplier governance including configurable:

```text
Prospective
Under Verification
Approved
Active
Restricted
Suspended
Blocked
Blacklisted
Inactive
Closed
```

Institutions are not required to use every state.

---

## 4.11 Multi-Campus Eligibility

A Supplier normally has one enterprise identity.

```text
               Supplier J02-SUP-001
                        │
          ┌─────────────┼─────────────┐
          ▼             ▼             ▼
       Campus A      Campus B      Campus C
       Eligible      Eligible      Restricted
```

J02 shall not create separate Supplier identities merely because the supplier operates at several campuses.

---

## 4.12 Duplicate Supplier Management

J02 shall support duplicate detection and controlled resolution.

Possible matching information may include:

* Legal Name
* Tax Identity
* Registration Identity
* Address
* Contact
* Bank Information
* External Supplier Identifier

Detection shall not automatically merge records.

---

## 4.13 External Procurement Suppliers

Suppliers originating from government or other external procurement systems shall be capable of mapping to canonical J02 Supplier identity.

External identity does not create a separate permanent Supplier Master.

---

## 4.14 Legacy Supplier Migration

Existing institutional supplier databases may be imported.

Institutions shall not be forced to make every historical supplier register again.

Migrated suppliers may undergo progressive verification and data cleansing.

---

# 5. Explicit Non-Ownership

## 5.1 J01 — Procurement Planning & Sourcing

J02 does not own:

* Procurement Planning
* RFQ
* RFP
* Tender
* Supplier Invitation
* Supplier Bid
* Supplier Quotation
* Technical Evaluation
* Commercial Evaluation
* Comparative Evaluation
* Supplier Recommendation
* Approved Sourcing Outcome

These belong to J01.

```text
J02
Supplier Identity + Eligibility
            │
            ▼
J01
Supplier Participation + Selection
```

---

## 5.2 J03 — Purchase Requisition & Approval

J02 does not own:

* Procurement Demand
* Purchase Requisition
* Requirement Approval
* Requester Justification

These belong to J03.

A Purchase Requisition may reference a suggested supplier where policy permits, but that does not transfer supplier selection to J03.

---

## 5.3 J04 — Purchase Order Management

J02 does not own:

* Purchase Order
* PO Amendment
* PO Cancellation
* Purchasing Commitment
* PO Terms
* PO Release

These belong to J04.

---

## 5.4 J05 — Goods Receipt & Inspection

J02 does not own:

* Goods Receipt
* Delivery Receipt
* Inspection
* Goods Acceptance
* Goods Rejection

J02 may consume resulting supplier-performance facts.

---

## 5.5 Inventory

J02 does not own:

* Item Master
* Stock
* Warehouse
* Stock Ledger
* Stock Transfer
* Inventory Valuation
* Stock Verification

These belong to the applicable Section J inventory architecture.

---

## 5.6 Assets

J02 does not own:

* Asset Registration
* Asset Assignment
* Asset Custody
* Asset Maintenance
* Asset Transfer
* Asset Disposal

Supplier references may remain part of asset procurement traceability.

---

## 5.7 Finance

J02 does not own:

* Supplier Invoice
* Accounts Payable
* Tax Accounting
* Payment
* Payment Authorization
* Financial Settlement
* General Ledger
* Budget

The ownership rule is:

```text
J02
Supplier Identity / Profile
          │
          ▼
Finance
Supplier Financial Transactions
```

---

## 5.8 Contract Lifecycle

J02 shall not become a general Contract Management system.

Supplier-related contractual evidence may be referenced where required for qualification or governance.

The existence of a Supplier relationship does not make J02 owner of procurement contracts.

---

# 6. Supplier Operating Model

## 6.1 Configurable Complexity

Supplier management shall support different institutional control levels.

### Simple College

```text
Supplier Created
      │
      ▼
Basic Details
      │
      ▼
Required Evidence
      │
      ▼
Verification
      │
      ▼
Active
```

### Standard University

```text
Registration
      │
      ▼
Duplicate Check
      │
      ▼
Documents
      │
      ▼
Verification
      │
      ▼
Category Qualification
      │
      ▼
Approval
      │
      ▼
Activation
```

### Highly Controlled Institution

```text
Registration
      │
      ▼
Identity Verification
      │
      ▼
Document Verification
      │
      ▼
Category Qualification
      │
      ▼
Bank Verification
      │
      ▼
Institutional Review
      │
      ▼
Approval
      │
      ▼
Activation
      │
      ▼
Ongoing Compliance Monitoring
```

These shall be configuration profiles rather than separate implementations.

---

## 6.2 Internal Registration

Authorized institutional users may create supplier registrations.

This is required for situations including:

* Existing suppliers
* Locally identified suppliers
* Suppliers discovered during procurement
* External procurement suppliers
* Service contractors
* Legacy supplier migration
* Administrative onboarding

The creator and registration source shall remain auditable.

---

## 6.3 Supplier Self-Registration

A future Supplier Portal may support:

```text
Supplier
    │
    ▼
Self-Registration
    │
    ▼
Prospective Supplier
    │
    ▼
Institutional Verification
    │
    ▼
Approval
```

Self-registration shall not automatically create an approved Supplier.

---

## 6.4 Prospective Supplier

J02 shall support prospective supplier identity where institutional policy allows it.

A prospective supplier may be permitted to:

* Complete onboarding.
* Upload documents.
* Declare categories.
* Receive permitted sourcing invitations.
* Participate in configured prequalification activity.

A prospective supplier shall not automatically have unrestricted Purchase Order eligibility.

---

## 6.5 Goods and Services

The same architecture shall support:

```text
Supplier
   │
   ├── Goods Supplier
   ├── Service Provider
   ├── Contractor
   ├── Consultant
   ├── Manufacturer
   ├── Dealer / Distributor
   ├── Maintenance Provider
   └── Professional Service Provider
```

These classifications shall not create separate master-data systems.

---

## 6.6 Enterprise Supplier Identity

Supplier identity shall normally be institution-wide.

Departments and campuses consume the same SupplierId.

Local eligibility and restrictions shall be modeled separately from identity.

---

# 7. Architectural Principles

## 7.1 One Real-World Supplier, One Canonical Identity

The architecture shall prevent intentional creation of separate canonical Supplier records merely because:

* Different campuses use the supplier.
* Different departments use the supplier.
* Finance and Procurement use different terminology.
* The supplier provides several categories of goods/services.

---

## 7.2 Registration Is Not Approval

```text
Registered
    ≠
Approved
```

Registration means the Supplier has entered the onboarding lifecycle.

---

## 7.3 Approval Is Not Procurement Selection

```text
Approved Supplier
       ≠
Selected Supplier
```

J02 determines eligibility.

J01 determines sourcing outcomes.

---

## 7.4 Qualification May Be Scoped

A Supplier may be:

```text
Qualified for IT Equipment

but

Not Qualified for Civil Works
```

Qualification shall therefore support category and scope.

---

## 7.5 Configuration Over Hard-Coding

Institutions shall configure:

* Required Fields
* Required Documents
* Verification Steps
* Approval Stages
* Qualification Requirements
* Sensitive Changes
* Campus Restrictions
* Supplier States

One institution's supplier policy shall not become hard-coded platform behavior.

---

## 7.6 Sensitive Changes Require Stronger Control

Sensitive supplier changes may require:

```text
Change Requested
      │
      ▼
Verification
      │
      ▼
Approval
      │
      ▼
New Value Effective
```

Examples include bank information and legal identity information.

---

## 7.7 No Generic Supplier Rating

J02 shall not reduce Supplier performance to public stars or a generic numeric rating.

Supplier performance shall remain evidence-based and private.

---

## 7.8 Performance Does Not Automatically Blacklist

Performance information may trigger:

```text
Review
Corrective Action
Restriction Review
Suspension Review
Blacklisting Review
```

It shall not automatically change the Supplier to Blacklisted.

---

## 7.9 Blacklisting Requires Governance

Blacklisting shall be an explicit institutional decision.

Where applicable, the record shall include:

* Reason
* Authority
* Evidence
* Effective Date
* Scope
* Review / End Date
* Decision History

---

## 7.10 Duplicate Detection Is Advisory

```text
Potential Duplicate
        │
        ▼
Controlled Review
      /       \
 Duplicate   Distinct
    │           │
    ▼           ▼
Resolution   Keep Both
```

Automated similarity detection shall not independently merge canonical identities.

---

## 7.11 Historical Integrity

Protected Supplier history shall not be destructively rewritten.

Material changes shall preserve applicable:

* Previous Value
* New Value
* Actor
* Timestamp
* Reason
* Approval
* Evidence

---

## 7.12 Cross-Domain Integrity

J02 shall expose Supplier information through stable contracts.

Other domains shall not update J02 through direct database access.

J02 shall likewise not directly mutate transactional aggregates owned by other domains.

---

# 8. Actors & Roles

## 8.1 Supplier Administrator

Responsible for operational Supplier Master administration.

Typical responsibilities include:

* Registration Administration
* Profile Maintenance
* Document Administration
* Duplicate Review Coordination
* Supplier Status Administration

---

## 8.2 Supplier Verification Officer

Responsible for configured verification activities.

The role may verify:

* Identity
* Documents
* Registration Details
* Tax Information
* Category Evidence
* Other Supplier Information

---

## 8.3 Supplier Approver

Authorized to approve Supplier onboarding according to institutional policy.

Approval authority may vary according to:

* Supplier Type
* Category
* Institution
* Campus
* Risk / Control Profile

---

## 8.4 Procurement Officer

Procurement officers consume Supplier information for J01.

They may:

* Search suppliers.
* initiate onboarding.
* request qualification.
* view permitted eligibility information.
* view permitted private performance history.

Procurement authority shall not automatically imply authority to approve sensitive Supplier changes.

---

## 8.5 Finance Verification Role

Finance or another authorized institutional function may participate in banking-information verification.

This participation does not transfer Supplier Master ownership to Finance.

---

## 8.6 Supplier Representative

A future Supplier Portal may allow an authorized supplier representative to:

* Maintain permitted profile information.
* submit onboarding information.
* upload documents.
* request category additions.
* request profile changes.
* request banking-information changes.
* view permitted onboarding status.

The Supplier Representative shall not approve their own institutional verification or eligibility.

---

## 8.7 Supplier Governance Authority

An institution may establish an authorized role or committee for:

* Restriction
* Suspension
* Blacklisting
* Reactivation
* Governance Exceptions

J02 references the authority but does not own employee identity or general committee administration.

---

## 8.8 Auditor / Reviewer

Authorized auditors may review:

* Supplier History
* Verification Evidence
* Approval History
* Sensitive Changes
* Duplicate Resolution
* Restriction / Suspension
* Blacklisting Decisions
* Reactivation
* Migration History

Audit visibility remains subject to data-access policy.

---

## 8.9 Segregation of Duties

Institutions may configure:

```text
Supplier Creator
      ≠
Supplier Approver
```

For sensitive changes:

```text
Change Requester
      ≠
Verifier
      ≠
Approver
```

For high-governance cases:

```text
Blacklisting Initiator
      ≠
Blacklisting Approver
```

Smaller institutions may configure simplified controls where permitted.

---

# 9. Supplier Identity Architecture

## 9.1 Canonical Supplier Identifier

Every canonical Supplier shall receive a stable `SupplierId`.

Example:

```text
SupplierId = J02-SUP-000123
```

The identifier remains stable when ordinary Supplier information changes.

---

## 9.2 Identity Attributes

Applicable identity information may include:

* SupplierId
* Legal Name
* Trade Name
* Supplier Type
* Organization Type
* Registration References
* Tax Identity References
* Primary Address
* Primary Contact
* Supplier Status
* Registration Source
* Creation Timestamp

Not every field shall be mandatory for every Supplier type.

---

## 9.3 Supplier Types

Configurable Supplier types may include:

```text
Organization
Individual Professional
Contractor
Consultant
Manufacturer
Dealer
Distributor
Service Provider
Other Institution-Defined Type
```

Supplier type shall not create a separate Supplier Master.

---

## 9.4 Legal Name versus Trade Name

J02 shall distinguish legal identity from presentation/trading identity where applicable.

```text
Legal Name
    │
    └── Canonical legal identity

Trade Name
    │
    └── Operational / commercial name
```

A trade-name change shall not automatically create a new Supplier.

---

## 9.5 External Identifiers

A Supplier may retain:

* Legacy Supplier Code
* External Procurement Supplier ID
* Government Portal Supplier Reference
* Institution-Specific Supplier Code
* Registration References

These are alternate identifiers.

They shall not replace `SupplierId`.

---

## 9.6 Identity-Changing Events

Some changes may indicate a different legal entity rather than an ordinary amendment.

Examples include:

* Legal Entity Replacement
* Merger
* Business Transfer
* Legal Organization Conversion
* Material Tax Identity Change

Such cases shall enter controlled identity review.

---

## 9.7 Identity Resolution

```text
Supplier Identity Change
          │
          ▼
   Same Legal Entity?
       /       \
     Yes       Unclear / No
      │             │
      ▼             ▼
   Amend        Identity Review
                    │
               ┌────┴────┐
               ▼         ▼
           Same Entity  New Entity
```

The outcome shall remain auditable.

---

# 10. Supplier Registration

## 10.1 Registration Sources

Supplier registration may originate from:

* Authorized Institutional User
* Supplier Self-Registration
* Procurement Process
* External Procurement Process
* Legacy Migration
* Administrative Import
* Approved Integration

The source shall be recorded.

---

## 10.2 Registration Lifecycle

A baseline lifecycle is:

```text
Registration Initiated
        │
        ▼
Draft
        │
        ▼
Submitted
        │
        ▼
Duplicate Check
        │
        ▼
Under Verification
        │
        ├────────► Information Required
        │
        ▼
Verified
        │
        ▼
Qualification / Approval
```

Institutions may simplify the lifecycle through configuration.

---

## 10.3 Minimum Registration Dataset

The minimum registration dataset shall be configurable.

A basic registration may contain:

* Supplier Name
* Supplier Type
* Contact
* Address
* Goods / Service Category
* Registration Source

Additional requirements may depend upon Supplier type or category.

---

## 10.4 Draft Registration

Incomplete registrations may remain Draft.

Draft registration does not create unrestricted procurement or purchasing eligibility.

---

## 10.5 Registration Submission

Submission indicates that the Supplier information is ready for institutional review.

Submission does not equal:

```text
Verification
Approval
Activation
```

---

## 10.6 Duplicate Screening

Before approval, J02 shall evaluate potential duplicates using available identity information.

Potential signals may include:

* Legal Name
* Tax Identity
* Registration Reference
* Address
* Contact
* Bank Information
* External Supplier Reference

The result may be:

```text
No Significant Match

Potential Duplicate

High-Confidence Duplicate Candidate
```

Even a high-confidence candidate shall require controlled resolution before identity merge.

---

## 10.7 Information Required

Where registration is incomplete:

```text
Submitted
    │
    ▼
Review
    │
    ▼
Information Required
    │
    ▼
Supplier / Staff Correction
    │
    ▼
Resubmitted
```

The previous submission shall remain traceable.

---

## 10.8 Registration Rejection

A registration may be rejected where institutionally appropriate.

The record shall preserve:

* Reason
* Decision Actor
* Decision Timestamp
* Evidence where applicable
* Registration Version

Rejection shall not be implemented through destructive deletion.

---

## 10.9 Registration Conversion

A successful registration shall eventually resolve to a canonical Supplier identity.

```text
SupplierRegistration
        │
        ▼
Verification / Approval
        │
        ▼
Canonical Supplier
        │
        ▼
SupplierId
```

Registration records remain onboarding evidence even after canonical Supplier creation.

```

This is **J02 Part 1**, covering the document foundation through **Section 10 — Supplier Registration**. 
```
````markdown
# 11. Supplier Onboarding

## 11.1 Purpose

Supplier onboarding governs the controlled transition of a Supplier from registration into an institutionally usable Supplier identity.

Onboarding shall coordinate the applicable:

- Identity Information
- Supplier Classification
- Contact Information
- Address Information
- Documents
- Tax / Regulatory Information
- Banking Information
- Category Information
- Verification Requirements
- Qualification Requirements
- Approval Requirements

Onboarding shall be configurable according to institutional policy.

---

## 11.2 Onboarding Lifecycle

A standard onboarding lifecycle is:

```text
Supplier Registration
        │
        ▼
Determine Onboarding Requirements
        │
        ▼
Collect Required Information
        │
        ▼
Collect Required Documents
        │
        ▼
Duplicate / Identity Review
        │
        ▼
Verification
        │
        ▼
Qualification
        │
        ▼
Approval
        │
        ▼
Activation
````

Not every institution shall be required to execute every stage.

---

## 11.3 Configurable Onboarding Requirements

Onboarding requirements may vary according to:

* Institution
* Supplier Type
* Supplier Category
* Goods / Service Classification
* Campus
* Procurement Policy
* Regulatory Requirement
* Institutional Risk Classification
* Supplier Registration Source

For example:

```text
Stationery Supplier
        │
        ▼
Basic Supplier Verification
```

may differ from:

```text
Security Service Contractor
        │
        ▼
Registration Verification
        │
        ▼
Licence / Compliance Evidence
        │
        ▼
Service Category Qualification
        │
        ▼
Institutional Approval
```

The examples represent configurable policy rather than hard-coded Supplier types.

---

## 11.4 Onboarding Checklist

J02 shall support a dynamically generated onboarding checklist.

Example:

```text
Supplier Onboarding Checklist

[x] Identity Information
[x] Contact Information
[x] Address
[x] Required Registration Information
[x] Tax Information
[x] Required Documents
[ ] Bank Verification
[x] Category Declaration
[ ] Category Qualification
[ ] Final Approval
```

Checklist requirements shall derive from active institutional configuration.

---

## 11.5 Incomplete Onboarding

A Supplier whose onboarding requirements remain incomplete may remain in a state such as:

```text
Onboarding In Progress
```

or:

```text
Information Required
```

The Supplier shall not automatically receive unrestricted eligibility merely because some onboarding steps are complete.

---

## 11.6 Returned Onboarding

Authorized reviewers may return onboarding for correction.

The return shall record:

* Reason
* Required Information
* Reviewer
* Timestamp
* Applicable Onboarding Version

Correction shall create a new submission state while preserving prior information for audit where required.

---

## 11.7 Onboarding Completion

Onboarding completion means all configured prerequisites required to proceed to approval or activation have been satisfied.

It shall not bypass configured approval.

```text
Onboarding Complete
        │
        ▼
Approval Required?
      /     \
    Yes      No
     │        │
     ▼        ▼
 Approval   Activation Eligibility
```

---

# 12. Supplier Verification

## 12.1 Purpose

Supplier verification establishes institutional confidence that submitted Supplier information and supporting evidence satisfy configured verification requirements.

Verification shall be evidence-based and auditable.

---

## 12.2 Verification Dimensions

Verification may include:

### Identity Verification

Verification of the Supplier's claimed legal or institutional identity.

### Registration Verification

Verification of applicable business or professional registration information.

### Tax Information Verification

Verification of applicable tax identity information.

### Address Verification

Verification of applicable registered or operating addresses.

### Contact Verification

Verification of authorized Supplier contact information.

### Document Verification

Verification of required supporting documents.

### Banking Verification

Verification of banking/profile information according to institutional policy.

### Category Verification

Verification of Supplier capability or evidence relevant to requested Supplier categories.

---

## 12.3 Verification Requirement Model

Verification requirements shall be configurable.

```text
Supplier
   │
   ▼
Supplier Type + Categories + Policy
   │
   ▼
Verification Requirement Set
   │
   ├── Identity Verification
   ├── Document Verification
   ├── Tax Verification
   ├── Bank Verification
   └── Category Verification
```

A verification requirement shall not be globally mandatory merely because the platform supports it.

---

## 12.4 Verification Status

A verification item may use states such as:

```text
Pending
Under Review
Information Required
Verified
Failed
Expired
Superseded
```

The exact operational vocabulary may be institution-configurable while canonical semantic meanings remain stable.

---

## 12.5 Verification Evidence

A verification decision shall retain applicable:

* SupplierId / RegistrationId
* Verification Type
* Evidence Reference
* Submitted Value
* Verification Outcome
* Verifier
* Verification Timestamp
* Reason / Remarks
* Validity Period where applicable

---

## 12.6 Verification Failure

A failed verification shall not automatically delete or blacklist the Supplier.

Possible outcomes include:

```text
Verification Failed
        │
        ├──► Request Correction
        │
        ├──► Request Additional Evidence
        │
        ├──► Reject Onboarding
        │
        └──► Escalate for Review
```

The applicable route shall depend on institutional policy.

---

## 12.7 Reverification

Reverification may be required when:

* A document expires.
* Sensitive Supplier information changes.
* Legal identity information changes.
* Institutional policy changes.
* A qualification is renewed.
* A governance review requires it.
* Imported legacy information requires progressive verification.

Reverification shall not erase previous verification history.

---

## 12.8 Verification versus Approval

Verification establishes that information/evidence has satisfied applicable checks.

Approval establishes institutional authorization to use the Supplier.

Therefore:

```text
Verified
   ≠
Approved
```

unless an institution explicitly configures a simplified workflow in which successful verification is itself the authorized approval action.

---

# 13. Supplier Approval & Activation

## 13.1 Supplier Approval

Supplier approval is the institutional decision that a Supplier has satisfied applicable onboarding requirements and may become eligible for authorized institutional use.

A baseline workflow is:

```text
Onboarding Complete
        │
        ▼
Submitted for Approval
        │
        ▼
Approval Review
      /   |   \
     /    |    \
Approve Return Reject
   │      │      │
   ▼      ▼      ▼
Approved Correction Rejected
```

---

## 13.2 Approval Authority

Approval authority may depend upon:

* Institution
* Supplier Type
* Supplier Category
* Campus
* Organizational Scope
* Institutional Delegation
* Governance Policy

J02 shall not hard-code one universal approver.

---

## 13.3 Approval Decision

An approval decision shall record:

* SupplierId / RegistrationId
* Decision
* Decision Authority
* Actor
* Timestamp
* Remarks / Reason where required
* Approval Scope
* Policy / Configuration Version

---

## 13.4 Approval Rejection

Supplier approval may be rejected.

Rejection shall retain the Supplier onboarding history and decision evidence.

Rejection shall not be implemented by deleting the Supplier registration.

---

## 13.5 Approval Return

Where correction is possible, the approver may return the Supplier onboarding case.

```text
Approval Review
      │
      ▼
Returned
      │
      ▼
Correction
      │
      ▼
Reverification where required
      │
      ▼
Resubmission
```

---

## 13.6 Activation

Activation makes an approved Supplier operationally available according to applicable eligibility rules.

```text
Approved
    │
    ▼
Activation Preconditions
    │
    ▼
Active
```

Institutions may configure automatic activation immediately after approval or require a separate activation action.

---

## 13.7 Active Supplier

An Active Supplier may become available to downstream domains according to eligibility.

Active status does not mean the Supplier is eligible for every category, campus or procurement method.

Example:

```text
Supplier Status: Active

IT Equipment: Qualified
Software Services: Qualified
Civil Works: Not Qualified

Campus A: Eligible
Campus B: Eligible
Campus C: Restricted
```

---

## 13.8 Approval versus Procurement Selection

Supplier approval shall never be interpreted as procurement award.

```text
J02 Supplier Approved
        │
        ▼
Eligible Supplier
        │
        ▼
J01 Sourcing Process
        │
        ▼
Supplier Evaluation / Selection
```

---

# 14. Supplier Classification & Categories

## 14.1 Purpose

Supplier classification allows the institution to describe the nature of the Supplier and the goods/services the Supplier is capable of providing.

Classification shall support institutional procurement without fragmenting Supplier identity.

---

## 14.2 Supplier Type

Supplier type describes the nature of the Supplier entity.

Examples may include:

* Organization
* Individual Professional
* Manufacturer
* Dealer
* Distributor
* Contractor
* Consultant
* Service Provider
* Agency
* Institution-Defined Type

---

## 14.3 Supplier Category

Supplier categories describe procurement capabilities.

Example:

```text
Supplier
   │
   ├── Laboratory Equipment
   ├── Laboratory Consumables
   ├── IT Equipment
   ├── Software
   ├── Civil Works
   ├── Electrical Works
   ├── Security Services
   ├── Housekeeping Services
   └── Consultancy
```

The actual category taxonomy shall be configurable.

---

## 14.4 Multiple Categories

One Supplier may belong to multiple categories.

The architecture shall not require separate Supplier records for each category.

---

## 14.5 Category Declaration versus Qualification

The architecture shall distinguish:

```text
Supplier Declares Category
          │
          ▼
Category Declared
          │
          ▼
Qualification Review
          │
          ▼
Category Qualified
```

A Supplier's declaration does not automatically establish institutional qualification.

---

## 14.6 Category Qualification State

A Supplier category relationship may contain:

* CategoryId
* Declaration Date
* Qualification Status
* Qualification Evidence
* Effective Date
* Expiry Date
* Restriction
* Campus Scope
* Verification Reference

---

## 14.7 Category Removal

A Supplier may request or institutionally undergo category removal.

Removal shall not erase historical procurement relationships.

The category relationship shall instead become inactive or end-dated.

---

## 14.8 Category Taxonomy Ownership

J02 may consume an enterprise procurement/resource classification taxonomy where one exists.

J02 owns the Supplier-to-category relationship, not necessarily the enterprise definition of every item/service category.

---

# 15. Supplier Document Management

## 15.1 Purpose

Supplier Document Management governs the business relationship between Supplier records and documents required for:

* Registration
* Verification
* Qualification
* Compliance
* Banking
* Governance

J02 shall not duplicate enterprise document-storage infrastructure.

---

## 15.2 Document Requirement

Document requirements may depend upon:

* Supplier Type
* Supplier Category
* Institution
* Campus
* Qualification
* Procurement Policy
* Regulatory Requirement

---

## 15.3 Supported Document Classes

Configurable document classes may include:

* Business Registration Evidence
* PAN-related Evidence where applicable
* GST-related Evidence where applicable
* MSME-related Evidence where applicable
* Bank Evidence
* Professional Registration
* Licence
* Certification
* Insurance Evidence
* Manufacturer Authorization
* Dealer Authorization
* Service Capability Evidence
* Institution-Specific Declaration
* Other Required Evidence

Support for a document type shall not make it universally mandatory.

---

## 15.4 Supplier Document Metadata

A Supplier document relationship may contain:

```text
SupplierDocumentReference
    ├── SupplierId
    ├── DocumentType
    ├── DocumentReference
    ├── DocumentNumber where applicable
    ├── IssuingAuthority
    ├── IssueDate
    ├── ExpiryDate
    ├── VerificationStatus
    ├── VerificationReference
    └── ConfidentialityClassification
```

---

## 15.5 Document Verification

Uploaded or referenced evidence may require verification.

```text
Document Submitted
       │
       ▼
Verification Required?
     /     \
   Yes      No
    │        │
    ▼        ▼
 Review    Accepted
    │
 ┌──┴───┐
 ▼      ▼
Verify Reject
```

---

## 15.6 Document Expiry

Documents with validity periods shall support expiry monitoring.

```text
Valid Document
      │
      ▼
Approaching Expiry
      │
      ├──► Notification
      │
      ▼
Expired
```

Expiry may affect Supplier qualification or eligibility according to configuration.

---

## 15.7 Expiry Notification

Configurable reminders may be generated before expiry.

For example:

```text
Document Expiry
      ▲
      │
90 Days
60 Days
30 Days
7 Days
```

The actual schedule shall be institution-configurable.

---

## 15.8 Expired Document Behaviour

Document expiry shall not automatically delete or blacklist a Supplier.

Configured outcomes may include:

* Warning Only
* Qualification Review
* Category Restriction
* New Procurement Restriction
* Supplier Suspension Review

The consequence shall be policy-driven.

---

## 15.9 Document Renewal

```text
Existing Document
       │
       ▼
Renewal Submitted
       │
       ▼
Verification
       │
       ▼
New Document Effective
```

Previous document references shall remain historically traceable.

---

# 16. Supplier Tax & Regulatory Information

## 16.1 Purpose

J02 shall maintain applicable Supplier tax and regulatory profile information required for Supplier identity, procurement and authorized downstream use.

J02 does not perform tax accounting.

---

## 16.2 Configurable Applicability

Tax and regulatory requirements vary by Supplier type and institutional context.

Therefore J02 shall not assume that every Supplier requires every supported identifier.

Applicable information may include:

* Tax Identity References
* GST-related Information where applicable
* PAN-related Information where applicable
* Registration Numbers
* Professional Registrations
* Regulatory Licences
* Institution-Specific Compliance References

---

## 16.3 Verification

Applicable tax/regulatory information may require verification.

Verification state shall be separately traceable from the raw submitted value.

---

## 16.4 Identity Impact

A material change to legal or tax identity may indicate:

```text
Ordinary Profile Correction
```

or:

```text
Possible Legal Entity Change
```

J02 shall route uncertain cases to Supplier Identity Review.

---

## 16.5 Finance Consumption

Finance may consume approved tax/profile information through authorized contracts.

Finance remains responsible for applying financial and tax rules to financial transactions.

---

# 17. Supplier Banking Information

## 17.1 Purpose

J02 shall maintain canonical approved Supplier banking/profile information required by authorized downstream financial processes.

Banking information is sensitive Supplier data.

---

## 17.2 Banking Profile

Applicable information may include:

* Account Holder Name
* Bank Name
* Branch Information
* Account Reference
* Routing / Banking Code Information
* Account Type where applicable
* Supporting Evidence
* Verification Status
* Effective Period

Actual fields shall be configurable according to institutional and banking requirements.

---

## 17.3 Access Control

Banking information shall not be generally visible to every procurement or institutional user.

Access shall be restricted according to role and business purpose.

---

## 17.4 Bank Information Change

A bank-information change shall be treated as a sensitive change.

Baseline workflow:

```text
Bank Change Requested
        │
        ▼
Evidence Submitted
        │
        ▼
Verification
        │
        ▼
Approval
      /     \
 Approve   Reject
    │
    ▼
New Bank Profile Effective
```

---

## 17.5 Maker-Checker Control

Institutions may require:

```text
Requester
    ≠
Verifier
    ≠
Approver
```

Simplified institutions may configure fewer stages where policy permits.

---

## 17.6 Historical Bank Information

Previous approved banking information shall remain historically traceable.

The system shall not silently overwrite bank information without audit history.

---

## 17.7 Finance Boundary

The canonical boundary is:

```text
J02
Approved Supplier Bank Profile
             │
             ▼
Finance
Payment / Settlement Processing
```

J02 does not:

* Initiate Payment
* Approve Payment
* Post Accounting Entries
* Determine Invoice Liability
* Mark an Invoice Paid

---

## 17.8 Downstream Safety

A bank-profile change shall emit an appropriate domain event.

Downstream Finance processes shall apply their own controls before using changed banking information for payment.

J02 approval of banking information shall not be interpreted as payment authorization.

---

# 18. Supplier Contact & Address Management

## 18.1 Supplier Contacts

A Supplier may maintain multiple contacts.

Contact purposes may include:

* Primary Contact
* Sales Contact
* Procurement Contact
* Accounts Contact
* Service Contact
* Support Contact
* Escalation Contact
* Authorized Representative

---

## 18.2 Supplier Addresses

A Supplier may maintain multiple addresses such as:

* Registered Address
* Corporate Address
* Branch Address
* Billing Address
* Service Address
* Correspondence Address

---

## 18.3 Primary Designation

J02 may designate a primary contact or address for a particular purpose.

Example:

```text
Supplier
   │
   ├── Registered Address
   ├── Billing Address
   └── Correspondence Address
```

---

## 18.4 Contact Changes

Supplier representatives may request permitted contact changes through the future Supplier Portal.

Institutional approval may be required for configured sensitive contact changes.

---

## 18.5 Historical Traceability

Where contacts or addresses were relevant to historical transactions, changes shall not destroy the ability to reconstruct the Supplier state applicable at that time.

---

# 19. Supplier Qualification & Eligibility

## 19.1 Purpose

Supplier qualification establishes whether a Supplier satisfies configured institutional requirements for a defined scope.

Eligibility determines whether the Supplier may currently be used within that scope.

These concepts shall remain distinct from Supplier identity.

---

## 19.2 Qualification Dimensions

Qualification may depend upon:

* Supplier Category
* Supplier Type
* Required Documents
* Certification
* Licence
* Experience Evidence
* Institutional Verification
* Campus
* Organizational Scope
* Validity Period

---

## 19.3 Qualification Lifecycle

```text
Qualification Requested
        │
        ▼
Requirements Evaluated
        │
        ▼
Evidence Reviewed
        │
    ┌───┼──────────┐
    ▼   ▼          ▼
Qualified  Conditional  Not Qualified
```

---

## 19.4 Qualification States

Canonical semantic states may include:

```text
Pending
Qualified
Conditionally Qualified
Expired
Suspended
Disqualified
```

Institutions may map these to local terminology.

---

## 19.5 Conditional Qualification

A Supplier may be conditionally qualified where institutional policy permits.

Conditions shall be explicit and auditable.

Example:

```text
Conditionally Qualified
        │
        ├── Limited Category
        ├── Limited Period
        └── Additional Evidence Required
```

---

## 19.6 Qualification Expiry

Qualification may have an expiry date.

Expiry shall trigger configurable:

* Notification
* Renewal
* Reverification
* Eligibility Restriction

Historical qualification shall remain traceable.

---

## 19.7 Eligibility

Eligibility is the current operational determination that a Supplier may participate in a defined institutional context.

Eligibility may consider:

```text
Supplier Status
      +
Qualification
      +
Document Validity
      +
Campus Scope
      +
Restrictions
      +
Blacklisting State
      =
Current Eligibility
```

---

## 19.8 J01 Consumption

J01 shall consume Supplier eligibility as decision context.

J01 remains responsible for determining whether a Supplier participates in or wins a sourcing process.

---

## 19.9 J04 Consumption

J04 may validate Supplier eligibility before creating or releasing a new Purchase Order according to procurement policy.

The exact purchasing rule belongs to J04.

J02 provides the authoritative Supplier eligibility facts.

---

# 20. Campus & Organizational Eligibility

## 20.1 Enterprise Identity Principle

A Supplier shall normally have one canonical enterprise identity even when used across multiple campuses.

```text
                 SupplierId
                     │
        ┌────────────┼────────────┐
        ▼            ▼            ▼
     Campus A     Campus B     Campus C
```

---

## 20.2 Campus Eligibility

Campus eligibility may be:

```text
Eligible
Restricted
Not Eligible
Pending
```

with applicable reason and effective dates.

---

## 20.3 Category-Specific Campus Eligibility

Eligibility may also be category-specific.

Example:

```text
Supplier
  │
  ├── Campus A
  │      ├── IT Equipment — Eligible
  │      └── Networking — Eligible
  │
  └── Campus B
         ├── IT Equipment — Eligible
         └── Networking — Restricted
```

---

## 20.4 Organizational Scope

Where required, eligibility may be scoped to:

* Campus
* College
* School
* Department
* Research Unit
* Administrative Unit
* Institution-Wide Scope

Organizational references shall use canonical organizational identifiers from the appropriate enterprise domain.

---

## 20.5 Local Restriction Does Not Change Identity

A Supplier restricted at one campus shall not require a new SupplierId for another campus.

```text
Identity
   ≠
Eligibility Scope
```

---

## 20.6 Eligibility History

Changes shall retain:

* Previous Eligibility
* New Eligibility
* Scope
* Reason
* Authority
* Effective Date
* End Date where applicable
* Audit Reference

---

# 21. Supplier Profile Amendment

## 21.1 Purpose

Supplier Profile Amendment governs changes to Supplier information after creation.

Changes shall be classified according to risk and sensitivity.

---

## 21.2 Ordinary Profile Changes

Examples may include:

* Trade Name
* Contact Details
* Correspondence Address
* Website
* Category Request
* Operational Contact

These may follow simplified approval according to policy.

---

## 21.3 Controlled Profile Changes

Examples may include:

* Legal Name
* Registered Address
* Tax Identity
* Banking Information
* Legal Entity Information
* Critical Regulatory Information

These may require verification and approval.

---

## 21.4 Change Request Model

```text
Current Supplier Profile
          │
          ▼
Change Requested
          │
          ▼
Change Classification
      /             \
Ordinary          Sensitive
   │                 │
   ▼                 ▼
Apply / Review    Verification
                     │
                     ▼
                  Approval
                     │
                     ▼
                   Apply
```

---

## 21.5 Effective Dating

Material Supplier changes may carry an effective date.

This supports accurate historical interpretation of Supplier information.

---

## 21.6 Amendment History

J02 shall preserve applicable:

```text
Old Value
New Value
Requester
Verifier
Approver
Reason
Effective Date
Timestamp
```

---

# 22. Sensitive Supplier Data Change

## 22.1 Purpose

Sensitive Supplier Data Change provides stronger governance for information whose unauthorized modification could create financial, legal, compliance or identity risk.

---

## 22.2 Sensitive Data Classification

Configurable sensitive data may include:

* Banking Information
* Legal Name
* Tax Identity
* Registration Identity
* Authorized Representative
* High-Risk Regulatory Information

---

## 22.3 Sensitive Change Workflow

```text
Sensitive Change Requested
          │
          ▼
Evidence Required
          │
          ▼
Verification
          │
          ▼
Approval
      ┌───┴────┐
      ▼        ▼
   Approved  Rejected
      │
      ▼
Apply New Value
      │
      ▼
Publish Change Event
```

---

## 22.4 Direct Editing Prohibition

Where a field is configured as controlled sensitive data, authorized users shall not bypass the change workflow through ordinary profile editing.

---

## 22.5 Rejection

Rejected changes shall preserve:

* Proposed Value
* Existing Value
* Reason
* Actor
* Timestamp
* Evidence Reference

The currently approved value remains effective.

---

## 22.6 Downstream Propagation

Approved material changes shall be communicated to authorized consuming domains through canonical events or APIs.

Consumers shall determine the impact on their own open transactions.

J02 shall not directly rewrite downstream transactional records.

```
```````markdown
# 23. Supplier Performance History

## 23.1 Purpose

J02 shall maintain a **private internal Supplier Performance History** to provide authorized institutional users with factual evidence about previous Supplier performance.

The capability exists to support informed procurement and supplier-governance decisions.

It shall not operate as a public Supplier rating or ranking system.

---

## 23.2 Core Principle

Supplier performance shall be represented through evidence and operational history rather than generic scores.

The architecture shall support:

```text
Supplier
    │
    ▼
Operational Experience
    │
    ├── Delivery Experience
    ├── Quality Experience
    ├── Service Experience
    ├── Inspection Outcomes
    ├── Rejection History
    ├── Corrective Actions
    └── Other Authorized Observations
    │
    ▼
Private Performance History
````

The following model is explicitly excluded:

```text
Supplier Rating = 4.2 / 5

Supplier Score = 78 / 100

Supplier Stars = ★★★★☆
```

---

## 23.3 Sources of Performance Evidence

Performance evidence may originate from authoritative domains including:

* J01 — Procurement Planning & Sourcing
* J04 — Purchase Order Management
* J05 — Goods Receipt & Inspection
* Service Acceptance capabilities
* Campus Operations
* Asset / Maintenance processes
* Other authorized institutional domains

J02 shall not fabricate operational performance information.

---

## 23.4 Performance Evidence Record

A performance record may contain:

```text
SupplierPerformanceRecord
    ├── PerformanceRecordId
    ├── SupplierId
    ├── PerformanceType
    ├── SourceDomain
    ├── SourceReference
    ├── ObservationDate
    ├── ObservationPeriod
    ├── Description
    ├── EvidenceReferences
    ├── RecordedBy
    ├── RecordedAt
    ├── ConfidentialityClassification
    └── Resolution / Follow-Up Reference
```

---

## 23.5 Performance Categories

Institutions may configure performance categories such as:

* Delivery
* Quality
* Service
* Responsiveness
* Support
* Warranty
* Compliance
* Corrective Action
* Procurement Experience
* Institution-Specific Category

These categories organize evidence.

They shall not automatically generate numeric Supplier ratings.

---

## 23.6 Positive and Negative Evidence

Performance history shall not exist solely for recording problems.

Authorized users may record applicable:

```text
Positive Performance Evidence
Neutral Operational Evidence
Adverse Performance Evidence
```

This provides a balanced historical record.

---

## 23.7 Private Visibility

Supplier performance history shall be considered internal institutional information.

Visibility may be restricted to:

* Authorized Procurement Personnel
* Supplier Governance Authorities
* Authorized Management
* Auditors
* Other Explicitly Authorized Roles

Supplier representatives shall not automatically have access to internal performance observations.

---

## 23.8 Performance Evidence versus Governance Decision

Performance history is evidence.

It is not itself a Supplier governance state.

```text
Adverse Performance Evidence
          │
          ▼
Institutional Review
          │
     ┌────┼─────────┐
     ▼    ▼         ▼
No Action Corrective Restriction /
          Action     Suspension Review
```

J02 shall require an explicit governance decision before changing Supplier status.

---

## 23.9 No Automatic Blacklisting

The following behavior is prohibited:

```text
Three Negative Records
        │
        ▼
Automatic Blacklisting
```

Instead:

```text
Performance Concern
        │
        ▼
Governance Review
        │
        ▼
Evidence Evaluation
        │
        ▼
Authorized Decision
```

---

## 23.10 Performance History Integrity

Performance evidence shall not be casually deleted merely because:

* The Supplier disputes it.
* The issue was resolved.
* A new procurement officer takes over.
* The Supplier later performs successfully.

Resolution or correction shall be recorded as additional history.

---

# 24. Supplier Issue & Incident History

## 24.1 Purpose

Supplier Issue & Incident History captures material Supplier-related operational issues that may require investigation, corrective action or governance review.

An issue or incident shall remain distinct from performance rating.

---

## 24.2 Incident Sources

Supplier incidents may originate from:

* Procurement
* Purchase Order Execution
* Delivery
* Inspection
* Service Delivery
* Campus Operations
* Maintenance
* Contractual Operations
* Authorized Administrative Review

---

## 24.3 Supplier Incident

A Supplier incident may contain:

```text
SupplierIncident
    ├── IncidentId
    ├── SupplierId
    ├── IncidentType
    ├── SourceDomain
    ├── SourceReference
    ├── IncidentDate
    ├── Description
    ├── EvidenceReferences
    ├── InstitutionalImpact
    ├── CorrectiveActionRequired
    ├── Status
    ├── Resolution
    └── AuditMetadata
```

---

## 24.4 Incident Lifecycle

```text
Incident Identified
        │
        ▼
Incident Recorded
        │
        ▼
Review
        │
   ┌────┼────────────┐
   ▼    ▼            ▼
Close  Corrective   Governance
       Action       Escalation
          │             │
          ▼             ▼
       Resolution     Review
```

---

## 24.5 Corrective Action

Where appropriate, an institution may request Supplier corrective action.

The corrective-action record may contain:

* Required Action
* Responsible Supplier Contact
* Due Date
* Supplier Response
* Supporting Evidence
* Review Outcome
* Closure Date

---

## 24.6 Incident Resolution

Resolution shall not erase the original incident.

```text
Incident
    │
    ▼
Corrective Action
    │
    ▼
Resolved
```

The complete history remains available to authorized users.

---

## 24.7 Incident Escalation

Material incidents may trigger consideration of:

* Category Restriction
* Campus Restriction
* Temporary Suspension
* Supplier Governance Review
* Blacklisting Proceedings

Such outcomes require their respective controlled workflows.

---

# 25. Supplier Suspension

## 25.1 Purpose

Supplier Suspension provides a controlled mechanism to temporarily prevent or limit new institutional use of a Supplier.

Suspension shall not destroy Supplier identity or historical transactions.

---

## 25.2 Suspension Reasons

Configurable suspension reasons may include:

* Pending Governance Investigation
* Serious Performance Concern
* Compliance Issue
* Expired Critical Documentation
* Supplier Request
* Legal / Regulatory Concern
* Institutional Policy Decision
* Other Authorized Reason

The architecture shall not hard-code specific legal conclusions.

---

## 25.3 Suspension Workflow

```text
Suspension Concern
        │
        ▼
Suspension Review
        │
        ▼
Evidence / Reason
        │
        ▼
Authorized Decision
      /          \
   Reject       Suspend
                  │
                  ▼
           Supplier Suspended
```

---

## 25.4 Suspension Record

A suspension shall contain applicable:

```text
SupplierRestriction
    ├── SupplierId
    ├── RestrictionType = Suspension
    ├── Reason
    ├── Authority
    ├── EffectiveFrom
    ├── EffectiveUntil
    ├── Scope
    ├── EvidenceReferences
    ├── DecisionReference
    └── ReviewStatus
```

---

## 25.5 Suspension Scope

Suspension may be:

```text
Institution-Wide
```

or, where institutional policy permits:

```text
Campus-Specific
Category-Specific
Organizational-Scope-Specific
```

Scope shall always be explicit.

---

## 25.6 Suspension Effect

A Supplier suspension may prevent configured activities such as:

* New Sourcing Invitations
* New Supplier Selection
* New Purchase Orders
* New Category Qualification

The exact effect shall be enforced by the consuming domain according to canonical J02 eligibility information.

---

## 25.7 Existing Transactions

Suspending a Supplier shall not automatically:

* Cancel Existing Purchase Orders
* Reverse Goods Receipts
* Cancel Supplier Invoices
* Reverse Payments
* Remove Existing Assets
* Delete Inventory History

Those decisions belong to the transactional domains that own those records.

---

## 25.8 Temporary Suspension

A suspension may have:

* Start Date
* Expected Review Date
* End Date
* Indefinite Pending Review

The system shall support review reminders where configured.

---

# 26. Supplier Blocking & Restriction

## 26.1 Purpose

Supplier Restriction allows institutions to limit specific Supplier activities without necessarily imposing complete suspension or blacklisting.

---

## 26.2 Restriction Types

Configurable restrictions may include:

* No New Sourcing Invitations
* No New Purchase Orders
* Category-Specific Restriction
* Campus-Specific Restriction
* Organizational Restriction
* Qualification Pending Restriction
* Document Expiry Restriction
* Bank Verification Restriction
* Institution-Defined Restriction

---

## 26.3 Restriction Model

```text
Supplier
    │
    ▼
Restriction
    │
    ├── Restriction Type
    ├── Scope
    ├── Reason
    ├── Effective Date
    ├── End / Review Date
    ├── Authority
    └── Evidence
```

---

## 26.4 Blocking versus Suspension

Institutions may use terminology such as `Blocked`, `Restricted`, or `Suspended`.

Canonical semantics shall distinguish:

```text
Restriction
    │
    └── Limits defined activities or scope

Suspension
    │
    └── Temporarily prevents broader Supplier use

Blacklisting
    │
    └── Formal governance prohibition according to policy
```

Local terminology may be mapped to these semantic states.

---

## 26.5 Automatic Operational Restriction

Some non-punitive restrictions may arise from objective configuration.

For example:

```text
Mandatory Qualification Expired
        │
        ▼
Category Not Currently Eligible
```

This is an eligibility consequence, not a disciplinary blacklisting decision.

---

## 26.6 Restriction Removal

Restrictions may be removed when their conditions are resolved.

```text
Restriction
     │
     ▼
Resolution Evidence
     │
     ▼
Review
     │
     ▼
Restriction Lifted
```

The historical restriction shall remain auditable.

---

# 27. Supplier Blacklisting

## 27.1 Purpose

Supplier Blacklisting represents a formal Supplier governance decision that prohibits Supplier use according to an explicitly defined institutional scope.

Blacklisting shall be treated as a high-governance action.

---

## 27.2 Blacklisting Principle

J02 shall never infer blacklisting merely from:

* Low Performance
* Number of Incidents
* Expired Documents
* User Complaint
* AI Recommendation
* Numeric Threshold

An explicit authorized decision is required.

---

## 27.3 Blacklisting Case

Blacklisting shall normally be represented through a governance case.

```text
Concern / Referral
        │
        ▼
Blacklist Case Opened
        │
        ▼
Evidence Collected
        │
        ▼
Governance Review
        │
        ▼
Decision
     /      \
Reject     Approve
  │           │
  ▼           ▼
Case       Supplier
Closed     Blacklisted
```

---

## 27.4 Blacklisting Record

A Blacklisting record shall preserve applicable:

* SupplierId
* BlacklistCaseId
* Reason
* Decision Authority
* Decision Date
* Effective Date
* Scope
* End / Review Date
* Evidence References
* Applicable Institutional Policy
* Decision Remarks
* Review / Appeal Reference where applicable

---

## 27.5 Blacklisting Scope

The scope shall be explicit.

Depending upon institutional policy, scope may include:

* Institution-Wide
* Campus
* Category
* Organizational Unit
* Defined Procurement Context

J02 shall not assume that every institution uses scoped blacklisting.

---

## 27.6 Downstream Effect

When a Supplier becomes blacklisted, J02 shall publish an authoritative Supplier governance event.

Relevant downstream domains shall apply their own rules.

Example:

```text
SupplierBlacklisted
        │
        ├────────► J01
        │          Prevent prohibited sourcing participation
        │
        ├────────► J04
        │          Apply rules to new purchasing commitments
        │
        └────────► Other Authorized Consumers
```

Existing legal/financial commitments shall not be silently destroyed by J02.

---

## 27.7 Blacklisting History

Blacklisting history shall remain traceable even after:

* Expiry
* Revocation
* Successful Review
* Reactivation

Historical governance decisions shall not be erased.

---

# 28. Supplier Reactivation

## 28.1 Purpose

Supplier Reactivation governs restoration of Supplier eligibility after an applicable restriction, suspension or inactive state.

---

## 28.2 Reactivation Preconditions

Depending upon the previous state, reactivation may require:

* Resolution of Cause
* Corrective Action Completion
* Updated Documents
* Reverification
* Qualification Renewal
* Governance Approval
* Bank Reverification
* Institutional Review

---

## 28.3 Reactivation Workflow

```text
Supplier Restricted / Suspended / Inactive
                  │
                  ▼
          Reactivation Requested
                  │
                  ▼
          Preconditions Checked
                  │
                  ▼
             Review Required?
              /          \
            Yes           No
             │             │
             ▼             │
          Approval         │
             │             │
             └──────┬──────┘
                    ▼
              Reactivated
```

---

## 28.4 Blacklisting Reactivation

Removal from blacklisting shall not use ordinary Supplier activation.

It shall require the authorized governance mechanism applicable to the blacklisting decision.

---

## 28.5 Historical State

Reactivation creates a new lifecycle transition.

It does not erase the previous:

* Restriction
* Suspension
* Blacklisting
* Inactivity

---

# 29. Supplier Inactivation & Closure

## 29.1 Purpose

Supplier Inactivation and Closure allow institutions to stop operational use of Suppliers that are no longer required without deleting Supplier history.

---

## 29.2 Inactivation Reasons

Examples may include:

* Supplier No Longer Used
* Supplier Business Closed
* Supplier Requested Inactivation
* Duplicate Resolution
* Legacy Supplier Cleanup
* Institutional Administrative Decision

Reason codes shall be configurable.

---

## 29.3 Inactivation

An inactive Supplier remains a valid historical identity.

```text
Active
   │
   ▼
Inactive
   │
   └── Historical references preserved
```

---

## 29.4 Closure

Closure may represent a more final lifecycle state where the Supplier relationship is considered concluded.

Closure shall not delete the Supplier.

---

## 29.5 Historical Transactions

Inactive or closed Suppliers may remain referenced by:

* Sourcing Events
* Purchase Orders
* Goods Receipts
* Inventory Transactions
* Assets
* Supplier Invoices
* Payments
* Audit Records

These references shall remain valid.

---

## 29.6 Reactivation from Inactive

Where permitted, an inactive Supplier may be reactivated after required review.

A closed Supplier may require stronger review or a new relationship decision depending upon policy.

---

# 30. Duplicate Supplier Detection & Resolution

## 30.1 Purpose

Duplicate Supplier Detection protects enterprise Supplier identity from fragmentation.

The architecture shall identify possible duplicates while avoiding unsafe automatic merging.

---

## 30.2 Duplicate Signals

Potential duplicate indicators may include:

* Legal Name Similarity
* Trade Name Similarity
* Tax Identity Match
* Registration Number Match
* Address Similarity
* Contact Match
* Email Match
* Phone Match
* Bank Information Match
* External Supplier ID Match
* Legacy Supplier Reference

---

## 30.3 Duplicate Detection Result

Detection may classify records as:

```text
No Match
Possible Match
Strong Match
Exact Identifier Match
```

These classifications support review.

They do not independently authorize merge.

---

## 30.4 Duplicate Review Workflow

```text
Potential Duplicate Detected
          │
          ▼
Identity Review
          │
    ┌─────┼──────────┐
    ▼     ▼          ▼
Distinct Duplicate  Insufficient
                    Information
    │       │          │
    ▼       ▼          ▼
Keep     Merge       Further
Both     Process     Review
```

---

## 30.5 Distinct Supplier Decision

Where records appear similar but represent different legal entities, the reviewer shall record the distinct-identity decision.

This prevents repeated false-positive review where appropriate.

---

## 30.6 Duplicate Confirmation

A confirmed duplicate shall not immediately cause destructive record deletion.

It shall proceed to controlled Supplier Merge & Identity Correction.

---

## 30.7 Duplicate Detection Technology

J02 may use:

* Exact Matching
* Rule-Based Matching
* Similarity Matching
* AI-Assisted Duplicate Detection

AI or similarity engines shall remain advisory.

The final identity decision requires authorized deterministic business action.

---

# 31. Supplier Merge & Identity Correction

## 31.1 Purpose

Supplier Merge resolves confirmed duplicate Supplier identities while preserving enterprise referential and historical integrity.

---

## 31.2 Surviving Supplier

A controlled merge shall identify:

```text
Duplicate Supplier A
          +
Duplicate Supplier B
          │
          ▼
Identity Review
          │
          ▼
Canonical Surviving Supplier
```

One SupplierId becomes the surviving canonical identity.

---

## 31.3 Merge Preconditions

A merge shall require:

* Confirmed Duplicate Decision
* Authorized Merge Actor
* Surviving Supplier Selection
* Attribute Reconciliation
* External Reference Reconciliation
* Downstream Impact Assessment where required

---

## 31.4 Merge Record

The merge shall retain:

```text
SupplierIdentityMerge
    ├── MergeId
    ├── SurvivingSupplierId
    ├── MergedSupplierIds
    ├── Reason
    ├── DecisionReference
    ├── ApprovedBy
    ├── MergedAt
    ├── AttributeResolution
    └── ReconciliationStatus
```

---

## 31.5 Historical Transaction Integrity

J02 shall not require destructive rewriting of every historical transaction.

Historical records may continue to contain the SupplierId originally used.

A canonical identity-resolution mapping shall allow:

```text
Legacy / Merged SupplierId
          │
          ▼
Canonical SupplierId
```

---

## 31.6 Conflicting Supplier Data

Where duplicate records contain conflicting information, the merge process shall explicitly determine the surviving value.

Sensitive data conflicts may require reverification.

---

## 31.7 Merge Irreversibility

A completed merge shall be treated as a high-governance identity action.

Any correction to an erroneous merge shall require an explicit identity-correction workflow rather than casual reversal.

---

# 32. External / Government Procurement Supplier Mapping

## 32.1 Purpose

J02 shall support Suppliers originating from external procurement systems, government procurement platforms, consortium systems or other approved procurement channels.

External Supplier identity shall be mapped into SutraERP's canonical Supplier model.

---

## 32.2 External Supplier Discovery

An external sourcing outcome may identify a Supplier not currently known to J02.

```text
External Procurement Outcome
          │
          ▼
External Supplier
          │
          ▼
Search J02
       /      \
   Found      Not Found
     │           │
     ▼           ▼
   Map       Register
     │           │
     └─────┬─────┘
           ▼
   Canonical SupplierId
```

---

## 32.3 Existing Supplier Mapping

Where the external Supplier matches an existing Supplier:

```text
ExternalSupplierId
        │
        ▼
ExternalSupplierMapping
        │
        ▼
SupplierId
```

The external identifier remains traceable.

---

## 32.4 New External Supplier

Where no Supplier exists, J02 shall create or initiate a Supplier registration according to policy.

Trusted external evidence may accelerate configured verification steps.

It shall not bypass identity integrity.

---

## 32.5 External Authority Recognition

Institutions may configure certain external authorities as trusted sources for specific evidence.

Example:

```text
Trusted External Evidence
          │
          ▼
Accepted Verification Input
```

This does not automatically imply that every internal Supplier approval requirement is waived.

---

## 32.6 Purchase Order Boundary

Before normal J04 purchasing, the external Supplier shall resolve to the applicable canonical SupplierId.

This ensures that external procurement does not create a parallel Supplier Master.

---

# 33. Existing Supplier Migration & Progressive Verification

## 33.1 Purpose

J02 shall support migration of Supplier Masters from existing institutional systems during SutraERP implementation.

ERP adoption shall not require every historical Supplier to start from zero.

---

## 33.2 Migration Sources

Supplier data may originate from:

* Legacy ERP
* Finance System
* Procurement System
* Spreadsheet
* Campus Database
* Departmental System
* Other Authorized Institutional Source

---

## 33.3 Migration Lifecycle

```text
Legacy Supplier Data
        │
        ▼
Import Validation
        │
        ▼
Duplicate Detection
        │
        ▼
Supplier Import
        │
        ▼
Migration Classification
        │
        ├── Verified Legacy
        ├── Verification Pending
        ├── Limited Use
        ├── Duplicate Review
        └── Inactive Legacy
```

---

## 33.4 Legacy Supplier Identifier

Legacy identifiers shall be preserved.

Example:

```text
LegacyVendorCode = VEN-4928
        │
        ▼
SupplierId = J02-SUP-001238
```

This supports reconciliation with historical systems and migrated transactions.

---

## 33.5 Progressive Verification

Institutions may progressively verify imported Suppliers.

```text
Imported Supplier
       │
       ▼
Operationally Required?
     /       \
   Yes        No
    │          │
    ▼          ▼
Priority     Deferred
Verification Verification
```

This avoids blocking ERP implementation because thousands of historical Suppliers require immediate re-onboarding.

---

## 33.6 Migration Data Quality

Migration may identify:

* Missing Information
* Invalid Information
* Duplicate Suppliers
* Inconsistent Names
* Missing Documents
* Obsolete Suppliers
* Invalid Contacts

These conditions shall be recorded for controlled remediation.

---

## 33.7 Historical Suppliers

Suppliers required only for historical transaction integrity may remain inactive legacy Suppliers.

They need not necessarily undergo full current onboarding unless they are reactivated for new procurement.

---

# 34. Supplier Portal Architecture

## 34.1 Purpose

The Supplier Portal is a future supplier-facing interaction channel for J02 and related authorized procurement capabilities.

The portal shall not create a separate Supplier domain.

---

## 34.2 Portal Capabilities

Future capabilities may include:

* Supplier Self-Registration
* Profile Maintenance Requests
* Contact Maintenance
* Address Maintenance
* Document Upload
* Document Renewal
* Category Requests
* Banking Change Requests
* Onboarding Status
* Verification Clarification
* Qualification Status where permitted
* Sourcing Invitations through J01 integration
* Supplier Responses through J01 integration

---

## 34.3 Portal Identity

Supplier Portal authentication belongs to the enterprise Identity & Access capability.

J02 shall maintain the relationship:

```text
Authenticated Supplier User
          │
          ▼
Authorized Supplier Representative
          │
          ▼
SupplierId
```

---

## 34.4 Supplier User Authorization

A Supplier representative may act only for authorized Supplier identities.

A user shall not gain access to another Supplier merely by knowing its SupplierId.

---

## 34.5 Self-Service Change Model

Supplier-entered information shall follow configured controls.

```text
Supplier Portal
      │
      ▼
Change Requested
      │
      ▼
J02 Validation
      │
      ├──► Directly Apply Permitted Low-Risk Change
      │
      └──► Verification / Approval for Controlled Change
```

---

## 34.6 Prohibited Self-Service Actions

Supplier users shall not directly:

* Approve their own Supplier registration.
* verify their own institutional evidence.
* qualify themselves.
* remove institutional restrictions.
* remove suspension.
* remove blacklisting.
* approve their own banking changes.
* alter internal performance history.
* alter audit history.

---

## 34.7 Portal Visibility

Supplier users may see only institutionally permitted information.

Private internal Supplier performance history shall not be exposed by default.

Internal governance evidence shall remain restricted.

---

## 34.8 Portal Independence

The Supplier Portal shall consume J02 application contracts.

It shall not directly access or modify J02 persistence.

This allows future portal replacement without changing canonical Supplier ownership.

```
``




