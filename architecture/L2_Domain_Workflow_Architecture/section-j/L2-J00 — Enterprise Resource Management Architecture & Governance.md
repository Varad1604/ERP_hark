# L2-J00 — Enterprise Resource Management Architecture & Governance

**Repository:** SutraERP
**Section:** J — Enterprise Resource Management
**Domain:** J00 — Enterprise Resource Management Architecture & Governance
**Document ID:** L2-J00
**Version:** 1.0
**Status:** Trust Certification Ready
**Classification:** Level 2 Section Architecture
**Owner:** Enterprise Resource Management
**Document Owner:** Chetan

---

# Table of Contents

1. Purpose
2. Section Mission
3. Architectural Position
4. Scope
5. Out of Scope
6. Business Objectives
7. Architectural Principles
8. Section J Domain Map
9. Enterprise Resource Lifecycle
10. Canonical Ownership Model
11. Domain Responsibility Matrix
12. Multi-Domain Boundaries
13. Enterprise Integration Model
14. Procurement Architecture
15. Supplier & Vendor Architecture
16. Purchase Requisition Architecture
17. Purchase Order Architecture
18. Goods Receipt & Inspection Architecture
19. Inventory & Warehouse Architecture
20. Stock Operations Architecture
21. Asset Lifecycle Architecture
22. Physical Verification & Audit Architecture
23. Disposal & Write-Off Architecture
24. Aggregate Catalogue
25. Command Catalogue
26. Query & Read Model Catalogue
27. Domain Event Catalogue
28. Consumed Event Catalogue
29. Multi-Domain Event Matrix
30. Business Rules & Invariants
31. Authorization & Segregation of Duties
32. Configuration Architecture
33. Audit & Traceability
34. Financial Integration
35. Campus Operations Integration
36. Academic, HR, Student & Examination Integration
37. Exception & Reconciliation Architecture
38. Reporting & KPI Responsibilities
39. Future Extension Points
40. Section-Level Architectural Constraints
41. Governance Decisions
42. Glossary
43. Canonical Ownership Matrix
44. Dependency Matrix

---

# 1. Purpose

Section J — Enterprise Resource Management provides the canonical enterprise architecture for managing institutional physical resources throughout their lifecycle.

The section governs the processes through which institutional resources are:

- Requested
- Planned
- Sourced
- Procured
- Received
- Inspected
- Stored
- Reserved
- Issued
- Distributed
- Assigned
- Transferred
- Verified
- Returned
- Retired
- Disposed

Section J establishes a common enterprise resource-management capability that may be consumed by every applicable SutraERP domain.

It prevents individual domains from independently implementing procurement, inventory, warehouse, vendor, asset, or stock-management capabilities.

The objective is to maintain one authoritative enterprise model for institutional physical resources while allowing consuming domains to retain ownership of their own operational activities.

---

# 2. Section Mission

The mission of Enterprise Resource Management is:

> To provide a unified, auditable, configurable, and enterprise-wide system for acquiring, receiving, storing, distributing, tracking, verifying, and retiring institutional physical resources while preserving clear ownership boundaries between resource management, finance, human resources, academic operations, examinations, and campus operations.

Section J acts as an enterprise service provider.

It does not represent a single institutional department.

Depending on institutional structure, activities governed by Section J may be performed by:

- Central Purchase Department
- Stores Department
- Institutional Procurement Office
- Campus Store
- Department Store
- Asset Management Office
- Finance-linked Procurement Team
- Administrative Office
- Laboratory Store
- Medical Store
- Examination Store
- Outsourced Logistics Provider

The architecture shall therefore model **business capabilities rather than organisational departments**.

---

# 3. Architectural Position

Enterprise Resource Management is a peer enterprise section within SutraERP.

```text
                         SutraERP

                            ¦
        +-------------------+-------------------+
        ¦                   ¦                   ¦
        ?                   ?                   ?

   Enterprise           Institutional       Operational
   Foundation             Domains             Domains

        ¦                   ¦                   ¦
        +-------------------+-------------------+
                            ¦
                            ?

             J — Enterprise Resource Management

                            ¦
          +-----------------+-----------------+
          ¦                 ¦                 ¦
          ?                 ?                 ?

     Procurement         Inventory          Assets

          ¦                 ¦                 ¦
          +-----------------+-----------------+
                            ¦
                            ?

                Enterprise Resource Services
```

Section J shall integrate across SutraERP rather than belong exclusively to Finance or Campus Operations.

---

## 3.1 Relationship with Finance

Finance and Enterprise Resource Management are closely integrated but remain separate canonical ownership boundaries.

Finance owns financial truth.

Examples include:

- Budgets
- Accounting
- General Ledger
- Accounts Payable
- Payments
- Financial Posting
- Asset Accounting
- Depreciation Accounting
- Financial Reporting

Section J owns physical-resource truth.

Examples include:

- Procurement
- Vendor Operational Records
- Purchase Requisitions
- Purchase Orders
- Goods Receipts
- Item Master
- Warehouses
- Physical Stock
- Stock Movements
- Operational Asset Register
- Physical Verification
- Disposal Operations

Section J shall request financial validations and publish financial consequences through defined integration contracts.

It shall not become an accounting system.

---

## 3.2 Relationship with Campus Operations

Campus Operations consumes institutional resources.

Examples include:

- Maintenance Spare Parts
- Laboratory Consumables
- Hostel Supplies
- Transport Materials
- Medical Supplies
- Security Equipment
- Housekeeping Consumables
- Library Operational Supplies

Campus Operations owns the operational reason for resource consumption.

Section J owns the corresponding inventory, stock movement, resource allocation, or asset lifecycle where applicable.

A Campus Operations domain shall not maintain an independent canonical stock ledger.

---

## 3.3 Relationship with Human Resources

Section E owns:

- Employee Identity
- Employment
- Organisational Assignment
- Workforce Lifecycle

Section J may assign institutional assets or resources to employees.

Section J does not own employee records.

For example:

```text
Employee
   ¦
   ¦ reference
   ?
Asset Assignment
   ¦
   ?
Laptop / Equipment / Institutional Resource
```

Employee identity remains canonical to Section E.

Asset assignment remains canonical to Section J.

---

## 3.4 Relationship with Academic Operations

Academic domains determine the operational or academic requirement for resources.

Section J manages acquisition and physical-resource fulfilment.

Example:

```text
Laboratory Requirement
        ¦
        ?
Material Request
        ¦
        ?
Inventory Availability
       / \
      /   \
Available  Unavailable
   ¦           ¦
   ?           ?
Stock Issue   Procurement
```

Academic domains shall not duplicate inventory ownership.

---

## 3.5 Relationship with Examination Management

Examination Management may require:

- Answer Books
- Examination Stationery
- Seals
- Packaging Materials
- Printing Consumables
- Other Controlled Examination Materials

Section I owns examination usage and examination confidentiality rules.

Section J owns procurement, receipt, stock custody, issue, return, and stock reconciliation where those materials are managed as institutional inventory.

Access restrictions for sensitive examination inventory shall remain configurable.

---

# 4. Scope

Section J governs the following enterprise capabilities:

### Procurement

- Procurement Planning
- Sourcing
- RFQ / Quotation Processes
- Supplier Selection
- Procurement Recommendations

### Supplier Management

- Supplier Master
- Supplier Qualification
- Supplier Status
- Supplier Performance
- Operational Contract References

### Purchase Requisitions

- Resource Requests
- Purchase Requisitions
- Approval
- Consolidation
- Budget Validation Integration

### Purchase Orders

- Purchase Order Creation
- Approval
- Amendment
- Supplier Communication
- Partial Fulfilment
- Closure

### Goods Receipt

- Goods Receipt
- Delivery Recording
- Quantity Verification
- Quality Inspection
- Acceptance
- Rejection
- Return to Supplier

### Inventory

- Item Master
- Warehouse
- Storage Location
- Bin
- Stock Ledger
- Batch
- Lot
- Serial Number
- Expiry Reference
- Reservation
- Reorder Parameters

### Stock Operations

- Stock Issue
- Stock Return
- Stock Transfer
- Department Distribution
- Inter-Campus Transfer
- Consumption Recording
- Stock Adjustment

### Assets

- Operational Asset Register
- Asset Identification
- Asset Assignment
- Asset Location
- Asset Transfer
- Custody
- Warranty
- AMC Reference
- Operational Lifecycle
- Retirement

### Verification

- Physical Stock Verification
- Cycle Counting
- Asset Verification
- Variance Identification
- Reconciliation

### Disposal

- Obsolescence
- Scrap
- Disposal
- Auction
- Retirement
- Write-Off Request
- Disposal Verification

---

# 5. Out of Scope

Section J shall not own capabilities that already have canonical owners elsewhere in SutraERP.

## Financial Accounting

Section J does not own:

- General Ledger
- Accounts Payable
- Accounts Receivable
- Payment Execution
- Bank Reconciliation
- Financial Statements
- Accounting Journals

These remain Finance responsibilities.

---

## Budget Ownership

Section J may request budget availability and commitment validation.

It shall not own institutional budgets.

---

## Employee Management

Section J may reference employees as:

- Requesters
- Approvers
- Custodians
- Asset Holders
- Inspectors

Employee lifecycle ownership remains Section E.

---

## Campus Operational Execution

Section J may issue materials used for:

- Maintenance
- Cleaning
- Laboratory Work
- Medical Operations
- Transport Operations
- Security Operations

It shall not execute those operational activities.

---

## Academic Ownership

Section J may provide academic materials and equipment.

It shall not determine:

- Curriculum
- Teaching Requirements
- Academic Eligibility
- Examination Outcomes
- Laboratory Academic Procedures

---

## Workflow Infrastructure

Section J defines business workflow requirements.

The enterprise workflow mechanism remains a platform capability.

---

## Notification Delivery

Section J publishes notification-worthy business events.

Notification delivery remains with the enterprise notification capability.

---

# 6. Business Objectives

Enterprise Resource Management shall provide the institution with:

1. A single authoritative Item Master.
2. A single authoritative Supplier Master.
3. A single authoritative Warehouse Registry.
4. A single physical Stock Ledger.
5. A single operational Asset Register.
6. Traceable procurement from requirement through fulfilment.
7. Complete stock-movement traceability.
8. Enterprise-wide resource visibility.
9. Multi-campus inventory control.
10. Controlled asset assignment and custody.
11. Physical verification and reconciliation.
12. Controlled disposal and write-off.
13. Integration with financial accounting without duplicating Finance.
14. Integration with operational domains without duplicating their workflows.
15. Complete auditability of institutional resource movement.

---

# 7. Architectural Principles

## 7.1 Canonical Ownership

Every enterprise resource concept shall have exactly one canonical owner.

No consuming domain shall maintain an independent authoritative copy of:

- Supplier
- Item
- Warehouse
- Stock Balance
- Stock Ledger
- Purchase Order
- Goods Receipt
- Operational Asset

---

## 7.2 Physical Truth versus Financial Truth

Section J owns physical-resource truth.

Finance owns financial truth.

The two shall integrate through explicit business contracts and domain events.

---

## 7.3 Requester Does Not Become Resource Owner

A domain requesting a resource does not gain ownership of inventory architecture.

For example:

```text
G08 Housekeeping
       ¦
       ¦ Cleaning Material Requirement
       ?
J07 Stock Operations
       ¦
       ?
J06 Inventory
```

G08 owns the cleaning requirement.

J06/J07 own stock fulfilment and movement.

---

## 7.4 Configuration over Customisation

The architecture shall support institutions with:

- Central Procurement
- Decentralised Procurement
- Central Warehouse
- Department Stores
- Campus Stores
- Multiple Warehouses
- No Formal Warehouse
- Internal Procurement Teams
- Outsourced Procurement Support

through configuration rather than architectural redesign.

---

## 7.5 Manual Operations Must Remain Supported

Barcode, RFID, IoT, automated warehousing, and AI-assisted procurement may be introduced in the future.

None shall be mandatory for baseline operation.

The system shall support institutions that currently perform significant resource-management activities manually.

---

## 7.6 Event-Driven Integration

Multi-domain communication shall use published business contracts and domain events.

Direct modification of another domain's canonical data is prohibited.

---

## 7.7 Audit First

Every material resource transaction shall be traceable.

This includes:

- Request
- Approval
- Procurement
- Receipt
- Inspection
- Stock Entry
- Reservation
- Issue
- Return
- Transfer
- Adjustment
- Asset Assignment
- Verification
- Disposal

---

## 7.8 Multi-Campus by Design

Inventory and asset architecture shall support:

```text
Institution
    ¦
    +-- Campus A
    ¦      +-- Central Store
    ¦      +-- Department Store
    ¦
    +-- Campus B
    ¦      +-- Central Store
    ¦      +-- Laboratory Store
    ¦
    +-- Campus C
           +-- Medical Store
```

Inter-campus movement shall be treated as an auditable resource transaction.

---

## 7.9 Segregation of Duties

The architecture shall permit institutions to separate responsibilities such as:

```text
Requester
    ?
Approver
    ?
Procurement Officer
    ?
Receiver
    ?
Inspector
    ?
Store Officer
    ?
Finance
```

The same user shall not automatically receive all permissions merely because they participate in one stage.

Segregation rules shall remain configurable according to institutional policy.

---

## 7.10 Resource Lifecycle Integrity

Every physical resource shall have a traceable lifecycle appropriate to its classification.

A resource may be:

```text
Requested
    ?
Procured
    ?
Received
    ?
Stored
    ?
Issued / Assigned
    ?
Used / Maintained
    ?
Verified
    ?
Returned / Retired
    ?
Disposed
```

Not every resource shall pass through every stage.

Consumables, fixed assets, controlled materials, services, and non-stock purchases may follow different configured lifecycle paths.
# 8. Section J Domain Map

## Purpose

Section J is decomposed into bounded contexts according to the enterprise resource lifecycle.

Each J-domain owns a distinct business capability and shall communicate with other J-domains through explicit contracts, commands, queries, and domain events.

The decomposition prevents Procurement, Inventory, Assets, Finance, and operational consumers from becoming a single tightly coupled resource-management module.

---

## 8.1 Domain Catalogue

| Domain | Name | Primary Responsibility |
|---|---|---|
| J00 | Enterprise Resource Management Architecture & Governance | Section architecture, ownership, governance, multi-domain rules |
| J01 | Procurement Planning & Sourcing | Procurement strategy, sourcing, RFQ/RFP/tender and supplier selection |
| J02 | Vendor & Supplier Management | Supplier master, qualification, status, performance and contract references |
| J03 | Purchase Requisition & Approval | Purchase demand, requisitions, approval and budget-validation coordination |
| J04 | Purchase Order Management | Purchase orders, amendments, supplier acknowledgement, fulfilment and closure |
| J05 | Goods Receipt, Inspection & Acceptance | Delivery receipt, inspection, acceptance, rejection and supplier return |
| J06 | Inventory & Warehouse Management | Item master, warehouse structure, stock ledger, batches, serials and stock availability |
| J07 | Stock Operations & Distribution | Reservation, issue, return, transfer, distribution and consumption recording |
| J08 | Asset Lifecycle Management | Operational asset register, custody, assignment, transfer, warranty and retirement |
| J09 | Physical Verification, Audit & Reconciliation | Stock counts, asset verification, variance identification and reconciliation |
| J10 | Disposal, Scrap & Write-Off | Disposal, scrap, auction, retirement and write-off coordination |

---

## 8.2 J00 — Enterprise Resource Management Architecture & Governance

J00 defines the architectural contract governing Section J.

J00 owns:

- Section-Level Architecture
- Canonical Ownership Rules
- Resource Governance Principles
- Multi-Domain Integration Standards
- Section-Level Business Invariants
- Resource Classification Standards
- Governance Review
- Section-Level Compliance Rules

J00 shall not execute procurement, inventory, asset, verification, or disposal transactions.

---

## 8.3 J01 — Procurement Planning & Sourcing

J01 governs activities performed before a supplier commitment becomes a Purchase Order.

J01 owns:

- Procurement Planning
- Procurement Method Selection
- Sourcing Events
- Request for Quotation
- Request for Proposal
- Tender References
- Bid / Quotation Collection
- Commercial Comparison References
- Technical Evaluation Coordination
- Supplier Selection Recommendation
- Procurement Award Recommendation

J01 consumes approved procurement demand from J03 where applicable.

J01 references suppliers owned by J02.

J01 shall not own:

- Supplier Master
- Purchase Requisition
- Purchase Order
- Goods Receipt
- Stock
- Payment

Procurement methods shall remain configurable so that institutions may support simple quotation-based purchasing, formal tendering, rate contracts, direct purchasing, emergency purchasing, or other approved institutional methods.

---

## 8.4 J02 — Vendor & Supplier Management

J02 is the canonical owner of the enterprise Supplier Master.

J02 owns:

- Supplier Identity
- Supplier Business Information
- Supplier Classification
- Supplier Qualification
- Supplier Compliance Status
- Approved Supplier Status
- Suspension
- Blacklisting
- Supplier Performance
- Supplier Operational Contacts
- Supplier Documentation References
- Contract / Agreement Operational References

J02 shall provide one authoritative supplier identity reusable across Procurement, Finance, Campus Operations, and other consuming domains.

Finance may maintain finance-specific supplier information such as:

- Payment Terms
- Tax Accounting References
- Payables Account References
- Payment Status

Such financial information shall not create a duplicate Supplier Master.

---

## 8.5 J03 — Purchase Requisition & Approval

J03 governs formal demand to acquire resources that are not fulfilled directly from available stock.

J03 owns:

- Purchase Requisition
- Requisition Lines
- Requested Quantity
- Required Date
- Requesting Unit
- Procurement Justification
- Requisition Consolidation
- Requisition Approval
- Requisition Rejection
- Requisition Amendment
- Requisition Cancellation
- Budget Validation Reference
- Requisition Fulfilment Status

A Purchase Requisition may originate from:

- Department Requirement
- Inventory Replenishment
- Asset Requirement
- Campus Operations Requirement
- Academic Requirement
- Employee Onboarding Requirement
- Examination Requirement
- Administrative Requirement

J03 shall request budget validation from Finance where institutional policy requires it.

Budget ownership remains with Finance.

---

## 8.6 J04 — Purchase Order Management

J04 is the canonical owner of Purchase Orders.

J04 owns:

- Purchase Order
- Purchase Order Lines
- Supplier Commitment
- Ordered Quantity
- Delivery Schedule
- Delivery Location
- Purchase Order Approval
- Purchase Order Amendment
- Supplier Acknowledgement
- Partial Fulfilment Status
- Order Cancellation
- Order Closure

A Purchase Order shall reference:

- Approved Procurement Authority
- Supplier from J02
- Procurement / Sourcing Outcome where applicable
- Purchase Requisition where applicable
- Financial Validation Reference where applicable

J04 shall not own:

- Goods Receipt
- Supplier Invoice
- Payment
- Stock Ledger

---

## 8.7 J05 — Goods Receipt, Inspection & Acceptance

J05 governs physical receipt and acceptance of procured resources.

J05 owns:

- Goods Receipt
- Delivery Record
- Received Quantity
- Receipt Location
- Receipt Date
- Inspection Requirement
- Inspection Result
- Accepted Quantity
- Rejected Quantity
- Short / Excess Delivery Observation
- Damage Observation
- Supplier Return
- Acceptance Status

The canonical flow is:

```text
Purchase Order
      ¦
      ?
Supplier Delivery
      ¦
      ?
Goods Receipt
      ¦
      ?
Inspection
   +-----+
   ¦     ¦
Accept Reject
   ¦     ¦
   ?     ?
Stock / Asset    Supplier Return
```

Inspection shall remain configurable.

Institutions may configure certain low-risk goods for direct acceptance while requiring formal inspection for equipment, controlled materials, laboratory resources, or other designated categories.

---

## 8.8 J06 — Inventory & Warehouse Management

J06 is the canonical owner of enterprise inventory truth.

J06 owns:

- Item Master
- Item Classification
- Stock-Keeping Rules
- Warehouse Registry
- Storage Locations
- Bins
- Batch / Lot References
- Serial Tracking
- Expiry Tracking
- Stock Ledger
- Stock Balance
- Available Stock
- Reserved Stock
- Reorder Parameters
- Safety Stock Parameters
- Inventory Status

No other SutraERP domain shall maintain an independent canonical Stock Ledger.

J06 shall support both formal and lightweight inventory models.

An institution may configure:

```text
Formal Model

Warehouse
   ?
Storage Location
   ?
Bin
   ?
Stock
```

or:

```text
Simple Model

Campus / Department Store
          ?
         Stock
```

Bin-level management shall therefore not be mandatory.

---

## 8.9 J07 — Stock Operations & Distribution

J07 governs physical movement and fulfilment of inventory after stock exists within J06.

J07 owns:

- Stock Reservation
- Stock Issue
- Department Issue
- Stock Return
- Stock Transfer
- Inter-Warehouse Transfer
- Inter-Campus Transfer
- Resource Distribution
- Consumption Recording
- Operational Stock Adjustment Requests

J06 remains owner of the resulting Stock Ledger and balances.

The separation is:

```text
J07
Business reason and movement operation
                ¦
                ?
J06
Canonical stock consequence
```

For example:

```text
Housekeeping Material Request
            ¦
            ?
      Stock Reservation
            ¦
            ?
        Stock Issue
            ¦
            ?
       J06 Stock Ledger
```

---

## 8.10 J08 — Asset Lifecycle Management

J08 is the canonical owner of the operational Asset Register.

J08 governs resources that require individual lifecycle tracking beyond normal consumable inventory.

J08 owns:

- Asset Identity
- Asset Tag
- Asset Classification
- Asset Registration
- Asset Custody
- Asset Assignment
- Asset Location
- Asset Condition
- Asset Transfer
- Asset Return
- Warranty Reference
- AMC Reference
- Service History Reference
- Asset Status
- Operational Retirement

Typical assets may include:

- Computers
- Laboratory Equipment
- Projectors
- Furniture
- Vehicles
- Medical Equipment
- Security Equipment
- Electrical Equipment
- Machinery

Asset classification shall remain configurable.

J08 owns the **operational asset lifecycle**.

Finance owns the **financial accounting lifecycle**.

```text
               Asset

          +-------------+
          ¦             ¦
          ?             ?

     J08 Operational    Finance
        Lifecycle      Accounting

       Custody        Capitalisation
       Location       Asset Value
       Condition      Depreciation
       Assignment     Book Value
       Transfer       Financial Posting
```

Neither context shall replace the other.

---

## 8.11 J09 — Physical Verification, Audit & Reconciliation

J09 governs independent verification that recorded resources correspond with physical reality.

J09 owns:

- Verification Plan
- Physical Stock Count
- Cycle Count
- Asset Verification
- Count Sheet
- Verification Observation
- Variance Identification
- Missing Resource Observation
- Excess Resource Observation
- Damaged Resource Observation
- Reconciliation Recommendation
- Verification Closure

J09 shall not silently modify inventory balances.

A discrepancy shall follow an auditable process:

```text
Physical Verification
        ¦
        ?
Variance Detected
        ¦
        ?
Investigation
        ¦
        ?
Reconciliation Decision
        ¦
        ?
Approved Adjustment
        ¦
        ?
J06 Stock Ledger
```

The person performing verification may be different from the person responsible for the relevant store or asset custody.

---

## 8.12 J10 — Disposal, Scrap & Write-Off

J10 governs controlled end-of-life treatment of institutional resources.

J10 owns:

- Disposal Request
- Disposal Assessment
- Obsolescence Classification
- Scrap Classification
- Disposal Approval
- Auction Process Reference
- Disposal Method
- Disposal Execution
- Disposal Verification
- Write-Off Request
- Disposal Certificate Reference

Supported disposal methods may include:

- Scrap
- Auction
- Sale
- Donation
- Return to Supplier
- Recycling
- Destruction
- Transfer
- Other Institution-Approved Methods

Disposal method shall remain configurable.

Financial write-off posting remains owned by Finance.

---

# 9. Enterprise Resource Lifecycle

## 9.1 Canonical Lifecycle

Section J governs the enterprise physical-resource lifecycle.

```text
Requirement Identified
        ¦
        ?
Availability Determined
      /     \
     /       \
Available   Not Available
   ¦             ¦
   ?             ?
Reservation   Purchase Requisition
   ¦             ¦
   ¦             ?
   ¦          Approval
   ¦             ¦
   ¦             ?
   ¦          Sourcing
   ¦             ¦
   ¦             ?
   ¦       Purchase Order
   ¦             ¦
   ¦             ?
   ¦        Goods Receipt
   ¦             ¦
   ¦             ?
   ¦         Inspection
   ¦             ¦
   +-------------¦
                 ?
             Inventory
                 ¦
                 ?
        Issue / Distribution
                 ¦
          +-------------+
          ¦             ¦
          ?             ?
     Consumption       Asset
                          ¦
                          ?
                       Custody
                          ¦
                          ?
                     Verification
                          ¦
                          ?
                       Retirement
                          ¦
                          ?
                       Disposal
```

Not every resource follows the entire lifecycle.

---

## 9.2 Resource Lifecycle Classes

Section J shall distinguish between resource classes.

### Consumable Resource

Typical lifecycle:

```text
Procure
  ?
Receive
  ?
Store
  ?
Issue
  ?
Consume
```

Examples include:

- Stationery
- Cleaning Materials
- Laboratory Consumables
- General Medical Consumables
- Maintenance Consumables

---

### Durable Non-Capital Resource

Typical lifecycle:

```text
Procure
  ?
Receive
  ?
Store
  ?
Issue
  ?
Custody / Usage
  ?
Return / Disposal
```

---

### Trackable Asset

Typical lifecycle:

```text
Procure
  ?
Receive
  ?
Inspect
  ?
Register Asset
  ?
Assign
  ?
Transfer / Maintain
  ?
Verify
  ?
Retire
  ?
Dispose
```

---

### Controlled Resource

Controlled resources may require additional:

- Authorization
- Restricted Storage
- Issue Controls
- Custody Records
- Reconciliation
- Audit

Controlled-resource policies shall remain configurable.

---

### Non-Stock Purchase

Not every purchase shall enter inventory.

Examples may include:

- One-time direct-use materials
- Certain low-value items
- Resources delivered directly to an approved consumption point

The institution shall configure whether a purchased item requires stock receipt, direct issue, or asset registration.

---

### Service Procurement

Services may pass through procurement without becoming physical inventory.

Example:

```text
Service Requirement
       ?
Purchase Requisition
       ?
Sourcing
       ?
Purchase Order / Contract
       ?
Service Confirmation
       ?
Finance
```

Service procurement shall therefore remain supported without generating artificial stock.

---

# 10. Canonical Ownership Model

## 10.1 Ownership Principle

Canonical ownership means that one bounded context is authoritative for the lifecycle and state of a business concept.

Other domains may:

- Reference it
- Query it
- Subscribe to its events
- Maintain purpose-specific read models

They shall not establish competing authoritative records.

---

## 10.2 Core Ownership Matrix

| Business Concept | Canonical Owner |
|---|---|
| Procurement Plan | J01 |
| Sourcing Event | J01 |
| RFQ / RFP | J01 |
| Supplier Master | J02 |
| Supplier Qualification | J02 |
| Supplier Performance | J02 |
| Purchase Requisition | J03 |
| Purchase Order | J04 |
| Goods Receipt | J05 |
| Goods Inspection | J05 |
| Item Master | J06 |
| Warehouse | J06 |
| Storage Location | J06 |
| Stock Ledger | J06 |
| Stock Balance | J06 |
| Batch / Lot | J06 |
| Inventory Serial Tracking | J06 |
| Stock Reservation | J07 |
| Stock Issue | J07 |
| Stock Return | J07 |
| Stock Transfer | J07 |
| Operational Asset Register | J08 |
| Asset Assignment | J08 |
| Asset Custody | J08 |
| Asset Transfer | J08 |
| Physical Verification | J09 |
| Verification Variance | J09 |
| Disposal Request | J10 |
| Disposal Execution | J10 |
| Budget | Finance |
| General Ledger | Finance |
| Payment | Finance |
| Financial Asset Value | Finance |
| Depreciation Accounting | Finance |
| Employee | Section E |
| Campus / Department | Institution Foundation |
| Operational Consumption Purpose | Requesting Domain |

---

## 10.3 Item Master versus Domain-Specific Resource Definition

J06 owns the enterprise Item Master.

However, operational domains may own domain-specific definitions associated with an item.

Example:

```text
J06 Item

Hydrochloric Acid
Item Code: CHEM-001
Unit: Litre
Stock Controlled: Yes

            ¦
            ¦ reference
            ?

G02 Laboratory Definition

Chemical Classification
Lab Usage Rules
Experiment Restrictions
Safety Requirements
```

The Item Master does not replace specialist domain semantics.

Likewise, specialist domains shall not duplicate stock ownership.

---

## 10.4 Supplier Master versus Financial Supplier Account

J02 owns supplier identity.

Finance may own the supplier's financial account.

```text
J02 Supplier
     ¦
     +---------------+
     ?               ?

Procurement       Finance

Sourcing          Payables
Performance       Payment
Qualification     Accounting
```

A supplier shall not require separate duplicate identities merely because multiple enterprise domains interact with it.

---

## 10.5 Asset Register versus Financial Asset Register

J08 owns the operational representation of an asset.

Finance owns its accounting representation.

Both shall share a stable multi-domain reference.

Example:

```text
Operational Asset ID
        ¦
        +-------- J08
        ¦
        ¦   Location
        ¦   Custodian
        ¦   Condition
        ¦   Warranty
        ¦
        +-------- Finance

            Capitalized Value
            Depreciation
            Book Value
            Accounting Classification
```

---

# 11. Domain Responsibility Matrix

| Capability | J01 | J02 | J03 | J04 | J05 | J06 | J07 | J08 | J09 | J10 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Procurement Planning | O |  |  |  |  |  |  |  |  |  |
| Sourcing | O | R | R |  |  |  |  |  |  |  |
| Supplier Master |  | O |  | R | R |  |  |  |  | R |
| Purchase Requisition | R |  | O | R |  |  |  |  |  |  |
| Purchase Order | R | R | R | O | R |  |  |  |  |  |
| Goods Receipt |  | R |  | R | O | R |  | R |  |  |
| Item Master |  |  | R | R | R | O | R | R | R | R |
| Stock Ledger |  |  |  |  | R | O | R |  | R | R |
| Stock Movement |  |  |  |  |  | R | O |  | R | R |
| Asset Lifecycle |  |  | R |  | R | R | R | O | R | R |
| Physical Verification |  |  |  |  |  | R | R | R | O | R |
| Disposal |  |  |  |  |  | R | R | R | R | O |

**Legend**

- **O** — Canonical Owner
- **R** — References / Collaborates

No responsibility marked `R` grants canonical ownership.

---

# 12. Multi-Domain Boundaries

## 12.1 Finance Boundary

Section J may request:

- Budget Availability
- Budget Reservation / Commitment where supported
- Financial Approval Status
- Supplier Financial Status
- Capitalization Processing
- Accounting Posting
- Payment Status

Finance may consume:

- Approved Purchase Order
- Goods Accepted
- Supplier Return
- Stock Valuation Facts where required
- Asset Registered
- Asset Retired
- Disposal Approved

Section J shall never directly post to the General Ledger.

---

## 12.2 Campus Operations Boundary

Campus Operations may create resource demand.

Examples:

```text
G01 Maintenance
      ?
Spare Part Requirement

G02 Laboratory
      ?
Consumable Requirement

G03 Hostel
      ?
Hostel Supply Requirement

G04 Transport
      ?
Operational Material Requirement

G06 Health
      ?
Medical Supply Requirement

G07 Security
      ?
Security Equipment Requirement

G08 Housekeeping
      ?
Cleaning Material Requirement
```

Section J fulfils the physical-resource component.

The requesting G-domain continues to own why and how the resource is operationally used.

---

## 12.3 Human Resources Boundary

Employee onboarding, transfer, and separation may trigger resource-management processes.

Example:

```text
Employee Onboarded
       ¦
       ?
Resource Requirement
       ¦
       ?
Asset Assignment
```

Employee separation may trigger:

```text
Employee Separation
       ¦
       ?
Assigned Asset Query
       ¦
       ?
Return / Transfer / Exception
```

Section J shall not block or control the canonical HR lifecycle unless an explicit institutional policy requires resource-clearance integration.

---

## 12.4 Academic Boundary

Academic and laboratory domains may determine:

- Required Material
- Required Equipment
- Academic Purpose
- Quantity Requirement
- Required Date

Section J determines:

- Stock Availability
- Reservation
- Fulfilment
- Procurement Status
- Physical Issue

---

## 12.5 Examination Boundary

Section I owns:

- Examination Material Purpose
- Confidentiality Classification
- Examination Usage
- Custody Rules specific to examination execution

Section J owns applicable:

- Procurement
- Receipt
- Inventory
- Issue
- Return
- Reconciliation

Confidential examination resources may use stricter J06/J07 authorization policies without creating a separate inventory system.

---

## 12.6 Platform and Cross-Cutting Boundary

Section J shall consume enterprise capabilities for:

- Authentication
- Authorization Infrastructure
- Workflow Execution
- Notifications
- Documents
- Audit Infrastructure
- Integration Messaging
- Configuration
- Reporting / Dashboard Presentation where applicable

Section J owns the **business rules** determining when these capabilities are invoked.

It does not duplicate the underlying platform capability.
# 13. Enterprise Integration Model

## 13.1 Purpose

Section J operates as an enterprise resource service provider across SutraERP.

Integration shall preserve canonical ownership while allowing requesting domains to initiate resource requirements and receive fulfilment status.

Section J shall not require consuming domains to understand its internal aggregates.

Likewise, Section J shall not directly manipulate aggregates owned by consuming domains.

The preferred integration model is:

```text
Requesting Domain
       ¦
       ¦ Business Request / Domain Event
       ?
Section J Contract
       ¦
       ?
J-Domain Processing
       ¦
       ¦ Domain Event
       ?
Requesting Domain Read Model / Reaction
```

---

## 13.2 Integration Principles

Multi-domain integration shall follow these principles:

- Published contracts over shared databases
- Domain events over direct aggregate modification
- Stable enterprise identifiers
- Idempotent event consumption
- Explicit ownership
- Eventual consistency where immediate consistency is unnecessary
- Synchronous validation only where the business transaction requires an immediate decision
- Immutable integration audit
- Institution and campus context on applicable contracts
- Correlation identifiers for end-to-end traceability

---

## 13.3 Enterprise Section Integration Matrix

| Section | Section J Consumes | Section J Provides |
|---|---|---|
| A — Platform & Core | Identity, authorization, workflow infrastructure, configuration | Resource business operations |
| B — Institution Foundation | Institution, campus, department, organisational references | Resource allocation and availability references |
| C — Student Lifecycle | Student/resource requirement references where applicable | Student-issued resource references where applicable |
| D — Academic Operations | Academic and laboratory resource requirements | Material/equipment fulfilment |
| E — Human Resources | Employee, organisational assignment, lifecycle events | Employee asset/resource assignment |
| F — Financial Management | Budget validation, accounting status, payment status | Procurement, receipt, stock and asset financial facts |
| G — Campus Operations | Operational material/equipment requirements | Stock, material and asset fulfilment |
| H — Enterprise Services | Notification, document, reporting and integration capabilities | Resource events and governance information |
| I — Examination Management | Examination material requirements and control classifications | Controlled material fulfilment and reconciliation |

---

# 14. Procurement Architecture

## 14.1 Purpose

Procurement Architecture governs the acquisition of goods and services required by the institution.

Procurement shall support both structured enterprise purchasing and simpler institutional purchasing models.

The architecture shall not assume that every institution operates a dedicated procurement department.

---

## 14.2 Procurement Models

Institutions may configure:

- Centralised Procurement
- Campus-Level Procurement
- Department-Level Procurement
- Hybrid Procurement
- Committee-Based Procurement
- Direct Purchase
- Quotation-Based Purchase
- Tender-Based Procurement
- Rate Contract Procurement
- Emergency Procurement

The selected procurement model shall not change canonical ownership.

---

## 14.3 Canonical Procurement Flow

```text
Requirement
    ¦
    ?
Purchase Requisition
    ¦
    ?
Approval
    ¦
    ?
Budget Validation
    ¦
    ?
Procurement Method
    ¦
    +---------------+
    ?               ?
Direct Purchase    Sourcing
                    ¦
                    ?
               Supplier Selection
                    ¦
                    ?
              Purchase Order
```

Budget validation may be mandatory, optional, or unavailable depending on institutional configuration and Finance integration.

---

## 14.4 Sourcing Workflow

```text
Sourcing Requirement
        ¦
        ?
Sourcing Method Selected
        ¦
        ?
Eligible Suppliers Identified
        ¦
        ?
RFQ / RFP / Tender
        ¦
        ?
Responses Received
        ¦
        ?
Evaluation
        ¦
        ?
Recommendation
        ¦
        ?
Selection / Approval
```

---

## 14.5 Procurement Evaluation

Evaluation may consider:

- Technical Compliance
- Commercial Offer
- Delivery Capability
- Supplier Qualification
- Historical Performance
- Warranty
- Service Support
- Contract Conditions

Evaluation criteria and weighting shall remain configurable.

Section J shall not hard-code a lowest-price-wins policy.

---

## 14.6 Emergency Procurement

Institutions may configure an emergency procurement path.

```text
Emergency Requirement
        ¦
        ?
Emergency Authority
        ¦
        ?
Expedited Procurement
        ¦
        ?
Receipt
        ¦
        ?
Post-Facto Review
```

Emergency procurement shall never bypass audit history.

Post-facto approval requirements shall remain configurable.

---

# 15. Supplier & Vendor Architecture

## 15.1 Supplier Identity

J02 shall maintain one canonical supplier identity.

A supplier may participate in:

- Procurement
- Purchase Orders
- Goods Delivery
- Returns
- Contracts
- Asset Warranty
- AMC
- Finance Payables

All such interactions shall reference the same enterprise Supplier ID.

---

## 15.2 Supplier Lifecycle

```text
Prospective
    ¦
    ?
Registered
    ¦
    ?
Under Evaluation
    ¦
    ?
Approved
    ¦
    ?
Active
    ¦
    +--------------? Suspended
    ¦
    +--------------? Blacklisted
    ¦
    ?
Inactive
```

Lifecycle states shall remain configurable.

---

## 15.3 Supplier Qualification

Qualification may evaluate:

- Business Registration
- Tax Documentation
- Banking Reference
- Product / Service Categories
- Technical Capability
- Compliance Documents
- Certifications
- Institutional Eligibility
- Historical Performance

Section J stores operational qualification status.

Sensitive financial validation remains subject to Finance ownership where applicable.

---

## 15.4 Supplier Performance

Supplier performance may consider:

- Delivery Timeliness
- Quantity Accuracy
- Quality Acceptance
- Rejection Rate
- Order Fulfilment
- Responsiveness
- Contract Compliance

Supplier performance shall be derived from authoritative transactional facts where available.

Manual evaluation may remain supported.

---

## 15.5 Contract Boundary

J02 may maintain operational procurement contract references required for:

- Supplier Eligibility
- Contract Validity
- Rate Contract Usage
- Delivery Terms
- Service Coverage
- Warranty / AMC References

Legal document management and financial accounting consequences remain with their respective canonical capabilities.

---

# 16. Purchase Requisition Architecture

## 16.1 Purpose

Purchase Requisition represents approved institutional demand to acquire goods or services.

It shall not be confused with an internal stock request.

```text
Need Existing Stock?
       ¦
   +-------+
   ¦       ¦
  Yes      No
   ¦       ¦
   ?       ?
J07 Stock  J03 Purchase
Request    Requisition
```

This distinction is mandatory.

---

## 16.2 Requisition Sources

Purchase Requisitions may originate from:

- Manual Department Request
- Approved Resource Requirement
- Inventory Reorder
- Asset Requirement
- Maintenance Requirement
- Laboratory Requirement
- Hostel Requirement
- Medical Requirement
- Housekeeping Requirement
- Security Requirement
- Examination Requirement
- Employee Onboarding Requirement
- Administrative Requirement

The originating domain owns the business reason.

J03 owns the Purchase Requisition.

---

## 16.3 Purchase Requisition Lifecycle

```text
Draft
  ¦
  ?
Submitted
  ¦
  ?
Validation
  ¦
  ?
Approval
  ¦
  +--------? Rejected
  ¦
  ?
Budget Validation
  ¦
  ?
Approved for Procurement
  ¦
  ?
Sourcing / Purchase Order
  ¦
  ?
Fulfilled
  ¦
  ?
Closed
```

Cancellation and amendment shall be supported according to policy.

---

## 16.4 Requisition Consolidation

Multiple approved demands may be consolidated for procurement.

Example:

```text
Department A -- 20 Chairs --+
                            ¦
Department B -- 30 Chairs --+--? Consolidated Requirement
                            ¦
Department C -- 10 Chairs --+
                                      ¦
                                      ?
                                  60 Chairs
```

Traceability to every originating requisition shall be preserved.

---

## 16.5 Budget Integration

Where required:

```text
Purchase Requisition
        ¦
        ?
Finance Budget Validation
        ¦
     +-----+
     ¦     ¦
Available Insufficient
     ¦     ¦
     ?     ?
Continue  Hold / Reject /
          Override Workflow
```

J03 shall store the validation reference and result.

J03 shall not calculate authoritative institutional budget balances.

---

# 17. Purchase Order Architecture

## 17.1 Purpose

A Purchase Order represents the institution's authorised procurement commitment to a supplier.

J04 is the canonical owner of the Purchase Order.

---

## 17.2 Purchase Order Sources

A Purchase Order may originate from:

- Approved Sourcing Outcome
- Approved Purchase Requisition
- Rate Contract
- Direct Purchase
- Emergency Procurement
- Approved Institutional Procurement Mechanism

The source shall remain traceable.

---

## 17.3 Purchase Order Lifecycle

```text
Draft
  ¦
  ?
Prepared
  ¦
  ?
Approval
  ¦
  ?
Issued
  ¦
  ?
Supplier Acknowledged
  ¦
  ?
Partially Fulfilled
  ¦
  ?
Fully Fulfilled
  ¦
  ?
Closed
```

Alternative transitions may include:

- Amendment
- Suspension
- Cancellation
- Rejection by Supplier

---

## 17.4 Purchase Order Amendments

Amendments may affect:

- Quantity
- Delivery Schedule
- Delivery Location
- Commercial Terms Reference
- Item Specification
- Order Lines

Every amendment shall preserve:

- Original Version
- Changed Version
- Approver
- Effective Date
- Reason

Issued Purchase Orders shall not be silently overwritten.

---

## 17.5 Partial Fulfilment

The architecture shall support:

```text
PO Quantity: 100

Delivery 1: 40
      ?
Remaining: 60

Delivery 2: 30
      ?
Remaining: 30

Delivery 3: 30
      ?
Fulfilled
```

J04 owns order fulfilment status.

J05 owns each physical Goods Receipt.

---

# 18. Goods Receipt & Inspection Architecture

## 18.1 Purpose

Goods Receipt establishes authoritative evidence that physical goods have arrived at an institutional receiving location.

Receipt does not automatically mean acceptance.

---

## 18.2 Goods Receipt Lifecycle

```text
Delivery Arrived
       ¦
       ?
Receipt Recorded
       ¦
       ?
Quantity Verification
       ¦
       ?
Inspection Required?
     /     \
   No       Yes
   ¦         ¦
   ¦         ?
   ¦      Inspection
   ¦       /      \
   ¦    Accept   Reject
   ¦       ¦        ¦
   +-------¦        ?
           ¦    Return / Resolution
           ?
       Accepted
           ¦
           ?
 Inventory / Asset Processing
```

---

## 18.3 Receipt Variances

J05 shall support recording:

- Short Delivery
- Excess Delivery
- Damaged Delivery
- Incorrect Item
- Incorrect Specification
- Missing Documentation
- Batch / Serial Mismatch
- Expiry Concern
- Quality Failure

Resolution policies shall remain configurable.

---

## 18.4 Inspection

Inspection may be performed by:

- Stores Personnel
- Requesting Department
- Technical Expert
- Laboratory Representative
- Engineering Representative
- Medical Representative
- Inspection Committee
- Other Authorised Role

Personnel identity remains externally owned.

J05 owns the inspection record.

---

## 18.5 Acceptance Consequences

Accepted goods may result in:

```text
Accepted Goods
      ¦
      +--------? J06 Inventory Receipt
      ¦
      +--------? J08 Asset Registration
      ¦
      +--------? Direct Consumption / Delivery
```

The correct route depends on item classification and institutional policy.

---

## 18.6 Rejected Goods

Rejected goods shall not become available inventory.

```text
Rejected Goods
      ¦
      ?
Quarantine / Hold
      ¦
      ?
Supplier Resolution
   +-----------------+
   ¦                 ¦
Replacement       Return
   ¦                 ¦
   ?                 ?
Reinspection      Closure
```

Quarantine handling shall remain configurable.

---

# 19. Inventory & Warehouse Architecture

## 19.1 Purpose

J06 provides the authoritative physical inventory model for SutraERP.

Inventory represents physical quantities held by the institution and available for controlled movement or consumption.

---

## 19.2 Item Master

The Item Master shall support classification such as:

- Stock Item
- Non-Stock Item
- Consumable
- Durable Item
- Asset-Candidate Item
- Controlled Item
- Batch-Tracked Item
- Serial-Tracked Item
- Expiry-Tracked Item

Classification shall be configurable and may combine multiple characteristics.

---

## 19.3 Item Identity

An Item may contain:

- Item ID
- Item Code
- Item Name
- Description
- Category
- Unit of Measure
- Stock-Controlled Flag
- Batch-Controlled Flag
- Serial-Controlled Flag
- Expiry-Controlled Flag
- Asset-Candidate Flag
- Active Status

Domain-specific attributes remain with specialist domains.

---

## 19.4 Warehouse Model

The architecture supports:

```text
Institution
     ¦
     ?
Campus
     ¦
     ?
Warehouse / Store
     ¦
     ?
Storage Location
     ¦
     ?
Bin
```

Institutions shall not be required to use every level.

---

## 19.5 Warehouse Types

Examples may include:

- Central Store
- Campus Store
- Department Store
- Laboratory Store
- Medical Store
- Examination Store
- Maintenance Store
- Controlled Store

Warehouse types shall remain configurable.

---

## 19.6 Stock Ledger

Every stock-affecting transaction shall generate an immutable Stock Ledger entry.

Typical movement types include:

- Receipt
- Issue
- Return
- Transfer Out
- Transfer In
- Adjustment Increase
- Adjustment Decrease
- Disposal
- Reconciliation

Historical ledger entries shall never be edited to change stock truth.

Corrections shall occur through compensating transactions.

---

## 19.7 Stock Balance

Stock balance is derived from authorised Stock Ledger movements.

Conceptually:

```text
Opening Balance
      +
Accepted Receipts
      +
Returns
      +
Transfer In
      +
Approved Positive Adjustments
      -
Issues
      -
Transfer Out
      -
Disposal
      -
Approved Negative Adjustments
      =
Current Physical Stock
```

Reserved stock shall remain distinguishable from physically available stock.

---

## 19.8 Batch, Lot and Serial Tracking

Tracking shall be enabled only where required.

### Batch / Lot

Useful for:

- Chemicals
- Medicines
- Consumables
- Expiry-Controlled Goods

### Serial

Useful for:

- Computers
- Equipment
- Electronics
- Trackable Devices

The architecture shall not force serial or batch management for ordinary low-value supplies.

---

## 19.9 Expiry Management

Expiry-controlled items may support:

- Manufacturing Date
- Expiry Date
- Near-Expiry Threshold
- Expired Status
- Issue Restriction
- Disposal Trigger

Domain-specific safety rules remain with the relevant specialist domain.

---

## 19.10 Reorder Management

Inventory may support configurable:

- Minimum Stock
- Maximum Stock
- Reorder Level
- Reorder Quantity
- Safety Stock

A reorder condition may generate a procurement recommendation or Purchase Requisition request.

Automatic purchasing shall not be mandatory.

---

# 20. Stock Operations & Distribution Architecture

## 20.1 Purpose

J07 governs authorised movement of existing institutional stock.

J07 shall distinguish physical movement from procurement.

---

## 20.2 Internal Material Request

An internal material request asks for existing inventory.

```text
Department
    ¦
    ?
Material Request
    ¦
    ?
Availability
   / \
  /   \
Yes    No
 ¦      ¦
 ?      ?
Reserve Procurement Requirement
 ¦
 ?
Issue
```

---

## 20.3 Reservation

Reservation temporarily allocates available stock to an approved requirement.

Reservation does not represent physical issue.

```text
Available Stock
      ¦
      ?
Reserved
      ¦
      ?
Issued
```

Reservations may expire according to configuration.

---

## 20.4 Stock Issue

Stock Issue records physical release from institutional custody to:

- Department
- Operational Unit
- Employee
- Laboratory
- Hostel
- Examination Unit
- Maintenance Team
- Other Authorised Consumer

The consuming domain owns the purpose.

J07 owns the issue transaction.

J06 owns the resulting ledger consequence.

---

## 20.5 Stock Return

Unused or returnable resources may be returned.

Return validation may consider:

- Item Condition
- Quantity
- Batch
- Serial
- Original Issue
- Return Reason

Returned stock shall become available only after required acceptance checks.

---

## 20.6 Stock Transfer

Transfers may occur between:

- Bins
- Storage Locations
- Warehouses
- Departments
- Campuses

A transfer shall preserve source and destination traceability.

For controlled transfers:

```text
Transfer Requested
       ¦
       ?
Approved
       ¦
       ?
Dispatched
       ¦
       ?
In Transit
       ¦
       ?
Received
       ¦
       ?
Completed
```

Simpler institutions may configure immediate transfers.

---

## 20.7 Consumption Recording

Consumption represents the business acknowledgement that an issued consumable has been used.

Consumption detail shall remain proportionate to institutional needs.

Section J shall not force grassroots-level consumption recording where institutions only require stock issue as the final inventory transaction.

---

## 20.8 Adjustments

Stock adjustments shall require:

- Adjustment Reason
- Quantity
- Item
- Location
- Authorisation
- Audit Record

Adjustment reasons may include:

- Damage
- Breakage
- Measurement Difference
- Approved Correction
- Verification Reconciliation
- Other Configured Reason

Adjustments shall never be used to hide unexplained stock discrepancies.
# 21. Asset Lifecycle Architecture

## 21.1 Purpose

J08 governs the operational lifecycle of institutional assets from registration through custody, assignment, movement, verification, retirement, and disposal.

An asset represents a physical resource requiring individual lifecycle control beyond ordinary stock management.

J08 is the canonical owner of the **Operational Asset Register**.

Finance remains the canonical owner of financial asset accounting.

---

## 21.2 Asset Creation Sources

An asset may originate from:

- Procurement and Goods Receipt
- Donation
- Grant
- Institutional Transfer
- Opening / Legacy Asset Migration
- Approved Asset Discovery during Verification
- Other Institution-Approved Acquisition

Every asset shall retain its acquisition-source reference where available.

Assets shall not be required to originate exclusively from procurement.

---

## 21.3 Asset Registration

Asset registration may include:

- Asset ID
- Asset Tag
- Item Reference
- Asset Category
- Description
- Manufacturer
- Model
- Serial Number
- Acquisition Reference
- Goods Receipt Reference
- Campus
- Location
- Custodian
- Condition
- Warranty Reference
- AMC Reference
- Operational Status

Financial values shall remain Finance-owned where applicable.

---

## 21.4 Asset Lifecycle

```text
Asset Identified
      ¦
      ?
Registered
      ¦
      ?
Available
      ¦
      ?
Assigned / Deployed
      ¦
      ?
In Use
      ¦
      +--------? Transfer
      ¦
      +--------? Return
      ¦
      +--------? Maintenance Reference
      ¦
      +--------? Verification
      ¦
      ?
Retirement Proposed
      ¦
      ?
Retired
      ¦
      ?
Disposal
```

Not every asset shall pass through every optional state.

Lifecycle policies shall remain configurable.

---

## 21.5 Asset Assignment

Assets may be assigned to:

- Employee
- Department
- Laboratory
- Classroom
- Office
- Hostel
- Library
- Medical Unit
- Security Unit
- Transport Unit
- Campus Facility
- Other Authorised Institutional Unit

Assignment establishes operational custody.

It does not transfer canonical asset ownership outside J08.

---

## 21.6 Employee Asset Assignment

Employee identity remains owned by Section E.

J08 stores only the employee reference required for custody.

```text
Section E
Employee
   ¦
   ¦ Employee ID
   ?
J08 Asset Assignment
   ¦
   ?
Institutional Asset
```

Employee transfer or separation may trigger asset-return or reassignment workflows.

Resource clearance shall remain configurable and shall not automatically become a mandatory blocker for HR lifecycle completion unless institutional policy explicitly requires it.

---

## 21.7 Asset Location

Assets may be associated with:

- Institution
- Campus
- Building
- Floor
- Room
- Department
- Warehouse
- Operational Unit

Location master ownership remains with the appropriate institutional or facilities domain.

J08 maintains the asset's location reference.

---

## 21.8 Asset Transfer

Asset transfer may include:

- Employee-to-Employee
- Department-to-Department
- Room-to-Room
- Building-to-Building
- Campus-to-Campus
- Store-to-Department
- Department-to-Store

Transfer shall preserve:

- Previous Custodian
- New Custodian
- Previous Location
- New Location
- Transfer Date
- Transfer Reason
- Authorisation
- Asset Condition

Historical custody shall remain immutable.

---

## 21.9 Asset Return

Assets may be returned because of:

- Employee Separation
- Employee Transfer
- Temporary Assignment Completion
- Replacement
- Repair Requirement
- Department Closure
- Asset Retirement
- Administrative Decision

Returned assets may become:

```text
Returned
   ¦
   +--------? Available
   +--------? Reassigned
   +--------? Repair / Maintenance
   +--------? Hold
   +--------? Retirement
```

---

## 21.10 Asset Condition

Example configurable conditions include:

- New
- Good
- Working
- Fair
- Damaged
- Under Repair
- Unserviceable
- Obsolete

Condition and lifecycle status shall remain separate concepts.

---

## 21.11 Warranty and AMC

J08 may maintain operational references for:

- Warranty Provider
- Warranty Start Date
- Warranty End Date
- Warranty Terms Reference
- AMC Provider
- AMC Start Date
- AMC End Date
- Service Coverage Reference

Contract ownership shall remain with the appropriate procurement/vendor capability.

J08 consumes contract references for operational asset management.

---

## 21.12 Maintenance Boundary

J08 owns the asset lifecycle.

The appropriate operational domain may own maintenance execution.

For example:

```text
J08 Asset
    ¦
    ¦ Maintenance Required
    ?
G01 Facilities / Relevant Maintenance Capability
    ¦
    ?
Maintenance Performed
    ¦
    ¦ Maintenance Result
    ?
J08 Asset History Reference
```

J08 shall not duplicate specialist maintenance workflows.

---

## 21.13 Asset Retirement

Retirement indicates that an asset is no longer approved for normal operational use.

Reasons may include:

- End of Useful Life
- Obsolescence
- Damage
- Safety Concern
- Uneconomical Repair
- Replacement
- Administrative Decision

Retirement does not automatically mean disposal.

```text
Active Asset
     ¦
     ?
Retirement
     ¦
     ?
Retired Asset
     ¦
     ?
J10 Disposal Process
```

---

# 22. Physical Verification, Audit & Reconciliation Architecture

## 22.1 Purpose

J09 provides independent verification of recorded physical resources.

Verification determines whether inventory and asset records correspond with physical reality.

---

## 22.2 Verification Types

J09 may support:

- Annual Stock Verification
- Periodic Stock Verification
- Cycle Counting
- Surprise Verification
- Warehouse Verification
- Department Verification
- Asset Verification
- Custodian Verification
- Campus Verification
- Targeted Verification

Verification models shall remain configurable.

---

## 22.3 Verification Planning

A Verification Plan may define:

- Scope
- Institution
- Campus
- Warehouse
- Department
- Item Categories
- Asset Categories
- Verification Date
- Verification Team
- Count Method
- Approval Requirements

---

## 22.4 Stock Verification Workflow

```text
Verification Planned
        ¦
        ?
Scope Frozen / Snapshot Referenced
        ¦
        ?
Physical Count
        ¦
        ?
Recorded Quantity Compared
        ¦
      +---+
      ¦   ¦
    Match Variance
      ¦   ¦
      ¦   ?
      ¦ Investigation
      ¦   ¦
      ¦   ?
      ¦ Reconciliation Recommendation
      ¦   ¦
      +---¦
          ?
    Verification Closure
```

Operational stock movement need not always be completely stopped during verification.

Institutions may configure:

- Full Stock Freeze
- Controlled Movement
- Snapshot-Based Counting

---

## 22.5 Asset Verification Workflow

```text
Asset Register
      ¦
      ?
Verification Scope
      ¦
      ?
Physical Verification
      ¦
      +--------? Found
      +--------? Location Mismatch
      +--------? Custody Mismatch
      +--------? Condition Difference
      +--------? Not Found
                       ¦
                       ?
                   Investigation
```

Verification shall not silently change asset ownership, custody, or location.

Corrections require authorised transactions.

---

## 22.6 Variance Classification

Variances may include:

- Quantity Shortage
- Quantity Excess
- Missing Asset
- Unrecorded Asset
- Location Difference
- Custody Difference
- Batch Difference
- Serial Difference
- Condition Difference
- Damaged Stock
- Expired Stock

Variance categories shall remain configurable.

---

## 22.7 Reconciliation

Reconciliation may result in:

- No Adjustment
- Stock Adjustment Request
- Asset Record Correction
- Transfer Correction
- Disposal Recommendation
- Investigation
- Administrative Escalation
- Financial Impact Notification

J09 recommends or authorises reconciliation according to policy.

J06 remains responsible for canonical stock consequences.

J08 remains responsible for canonical asset consequences.

Finance remains responsible for financial consequences.

---

## 22.8 Verification Independence

Institutions shall be able to separate:

```text
Stock Custodian
      ?
Physical Verifier
      ?
Adjustment Approver
```

This segregation shall remain configurable according to institutional size and governance requirements.

Small institutions may operate simplified verification workflows while preserving auditability.

---

# 23. Disposal, Scrap & Write-Off Architecture

## 23.1 Purpose

J10 governs controlled removal of institutional resources from active inventory or asset lifecycle.

Disposal shall be explicit, authorised, traceable, and irreversible through ordinary editing.

Corrections shall use compensating or reversal workflows where institutionally permitted.

---

## 23.2 Disposal Sources

A disposal process may originate from:

- Asset Retirement
- Obsolete Inventory
- Damaged Inventory
- Expired Inventory
- Failed Verification
- Unserviceable Asset
- Surplus Resource
- Administrative Decision
- Safety Requirement
- Regulatory Requirement

---

## 23.3 Disposal Lifecycle

```text
Disposal Candidate
       ¦
       ?
Assessment
       ¦
       ?
Disposal Proposal
       ¦
       ?
Approval
       ¦
       ?
Disposal Method Selected
       ¦
       ?
Execution
       ¦
       ?
Verification
       ¦
       ?
Resource Closed
       ¦
       ?
Financial Consequence Published
```

---

## 23.4 Disposal Methods

Supported methods may include:

- Scrap
- Auction
- Sale
- Donation
- Recycling
- Destruction
- Return to Supplier
- Institutional Transfer
- Other Approved Method

Availability of disposal methods shall remain configurable.

---

## 23.5 Disposal Assessment

Assessment may consider:

- Resource Condition
- Remaining Utility
- Repair Feasibility
- Safety
- Obsolescence
- Storage Cost
- Regulatory Requirement
- Operational Requirement
- Financial Reference
- Disposal Restrictions

---

## 23.6 Auction

Where institutions conduct auctions, J10 may support:

- Auction Proposal
- Resource Lot
- Auction Approval
- Bid Reference
- Selected Buyer Reference
- Disposal Confirmation

J10 shall not become a general-purpose commercial marketplace.

Online auction integration may remain a future extension.

---

## 23.7 Scrap

Scrap processing may support:

- Scrap Classification
- Scrap Quantity
- Scrap Location
- Approval
- Handover
- Recipient Reference
- Disposal Evidence

Hazardous or regulated material disposal may require specialist-domain or external regulatory processes.

J10 shall coordinate the resource lifecycle without replacing those specialist controls.

---

## 23.8 Write-Off Boundary

Operational disposal and financial write-off are distinct.

```text
J10
Physical Disposal
      ¦
      ¦ AssetDisposed / InventoryDisposed
      ?
Finance
Financial Write-Off
      ¦
      ?
Accounting Closure
```

J10 shall not directly alter:

- Asset Book Value
- General Ledger
- Accumulated Depreciation
- Financial Loss
- Accounting Journals

---

# 24. Aggregate Catalogue

## 24.1 Purpose

Section J uses aggregates to protect transactional consistency while preventing oversized multi-domain transaction boundaries.

No transaction shall require direct atomic modification of aggregates belonging to multiple bounded contexts.

---

## 24.2 Section Aggregate Catalogue

| Domain | Aggregate Root | Primary Responsibility |
|---|---|---|
| J01 | ProcurementPlan | Procurement planning |
| J01 | SourcingEvent | RFQ/RFP/tender and sourcing lifecycle |
| J02 | Supplier | Supplier identity and qualification |
| J02 | SupplierEvaluation | Supplier performance evaluation |
| J03 | PurchaseRequisition | Procurement demand and approval |
| J04 | PurchaseOrder | Supplier purchasing commitment |
| J05 | GoodsReceipt | Physical receipt |
| J05 | GoodsInspection | Acceptance/rejection decision |
| J06 | Item | Enterprise Item Master |
| J06 | Warehouse | Warehouse/storage structure |
| J06 | InventoryPosition | Controlled inventory position |
| J07 | StockReservation | Stock allocation |
| J07 | StockIssue | Physical issue |
| J07 | StockTransfer | Physical transfer |
| J07 | StockReturn | Stock return |
| J08 | Asset | Operational asset lifecycle |
| J08 | AssetAssignment | Asset custody |
| J09 | VerificationPlan | Verification scope and execution |
| J09 | VerificationVariance | Identified physical discrepancy |
| J10 | DisposalCase | Resource disposal lifecycle |

Detailed aggregate boundaries shall be refined in the corresponding J01—J10 documents.

---

## 24.3 Aggregate Identity

Every aggregate shall use:

- Immutable System Identifier
- Institution Context
- Human-Readable Business Reference where required
- Version / Concurrency Information where required

Business references may be configurable by institution.

---

## 24.4 Cross-Aggregate References

Aggregates shall reference other aggregates by stable identifier rather than embedding another aggregate's mutable state.

Example:

```text
PurchaseOrder
    ¦
    +-- SupplierId
    +-- PurchaseRequisitionId
    +-- SourcingEventId
```

The Purchase Order shall not contain authoritative copies of those aggregates.

Necessary transactional snapshots may be retained for historical and legal evidence without changing canonical ownership.

---

## 24.5 Historical Snapshots

Certain business documents require preservation of the information that existed when they were issued.

Examples include:

- Purchase Order Supplier Name
- Delivery Address
- Item Description
- Commercial Terms
- Unit of Measure

Such snapshots are historical evidence.

They do not become competing master data.

---

# 25. Command Catalogue

## 25.1 Command Principles

Commands represent explicit business intent.

Commands shall:

- Be imperative
- Target one owning bounded context
- Be authorised
- Be validated
- Produce an auditable outcome
- Publish domain events after successful state transitions

---

## 25.2 J01 Procurement Commands

- CreateProcurementPlan
- UpdateProcurementPlan
- InitiateSourcingEvent
- IssueRFQ
- RecordSupplierResponse
- EvaluateSupplierResponse
- RecommendSupplier
- CloseSourcingEvent
- CancelSourcingEvent

---

## 25.3 J02 Supplier Commands

- RegisterSupplier
- UpdateSupplierProfile
- SubmitSupplierForQualification
- ApproveSupplier
- SuspendSupplier
- ReinstateSupplier
- BlacklistSupplier
- RecordSupplierEvaluation
- DeactivateSupplier

---

## 25.4 J03 Purchase Requisition Commands

- CreatePurchaseRequisition
- AddPurchaseRequisitionLine
- SubmitPurchaseRequisition
- ApprovePurchaseRequisition
- RejectPurchaseRequisition
- AmendPurchaseRequisition
- CancelPurchaseRequisition
- ConsolidatePurchaseRequisitions
- RequestBudgetValidation
- MarkRequisitionForProcurement

---

## 25.5 J04 Purchase Order Commands

- CreatePurchaseOrder
- SubmitPurchaseOrderForApproval
- ApprovePurchaseOrder
- IssuePurchaseOrder
- RecordSupplierAcknowledgement
- AmendPurchaseOrder
- CancelPurchaseOrder
- ClosePurchaseOrder

---

## 25.6 J05 Goods Receipt Commands

- RecordGoodsReceipt
- RecordDeliveryVariance
- RequestGoodsInspection
- RecordInspectionResult
- AcceptGoods
- RejectGoods
- PlaceGoodsOnHold
- ReleaseHeldGoods
- InitiateSupplierReturn
- CloseGoodsReceipt

---

## 25.7 J06 Inventory Commands

- CreateItem
- UpdateItem
- ActivateItem
- DeactivateItem
- CreateWarehouse
- CreateStorageLocation
- ConfigureItemTracking
- RecordInventoryReceipt
- PostStockLedgerMovement
- ConfigureReorderPolicy
- PlaceInventoryOnHold
- ReleaseInventoryHold

Direct stock-balance editing shall not be exposed as a command.

---

## 25.8 J07 Stock Operations Commands

- CreateMaterialRequest
- ReserveStock
- ReleaseReservation
- IssueStock
- ReturnStock
- RequestStockTransfer
- ApproveStockTransfer
- DispatchStockTransfer
- ReceiveStockTransfer
- RecordConsumption
- RequestStockAdjustment

---

## 25.9 J08 Asset Commands

- RegisterAsset
- TagAsset
- AssignAsset
- ReturnAsset
- TransferAsset
- UpdateAssetCondition
- UpdateAssetLocation
- RecordWarrantyReference
- RecordAMCReference
- ProposeAssetRetirement
- RetireAsset

---

## 25.10 J09 Verification Commands

- CreateVerificationPlan
- StartVerification
- RecordPhysicalCount
- RecordAssetVerification
- RecordVerificationVariance
- InvestigateVariance
- RecommendReconciliation
- ApproveReconciliation
- CloseVerification

---

## 25.11 J10 Disposal Commands

- CreateDisposalCase
- AssessDisposalCandidate
- RecommendDisposal
- ApproveDisposal
- SelectDisposalMethod
- RecordAuctionOutcome
- RecordScrapHandover
- ExecuteDisposal
- VerifyDisposal
- CloseDisposalCase

---

# 26. Query & Read Model Catalogue

## 26.1 Query Principles

Queries shall not mutate domain state.

Section J shall use CQRS-oriented read models where cross-aggregate or multi-domain information is required.

Read models may be eventually consistent.

---

## 26.2 Procurement Read Models

Examples include:

- Open Procurement Requirements
- Active Sourcing Events
- Pending Supplier Responses
- Procurement Cycle Status
- Procurement Pipeline
- Procurement by Department
- Procurement by Campus

---

## 26.3 Supplier Read Models

Examples include:

- Approved Supplier Directory
- Suspended Supplier List
- Supplier Qualification Status
- Supplier Performance Summary
- Supplier Delivery Performance
- Supplier Rejection Trends

---

## 26.4 Purchase Read Models

Examples include:

- Pending Purchase Requisitions
- Requisitions Awaiting Approval
- Budget Validation Pending
- Open Purchase Orders
- Partially Fulfilled Purchase Orders
- Purchase Orders Awaiting Delivery
- Purchase Order Fulfilment Summary

---

## 26.5 Goods Receipt Read Models

Examples include:

- Expected Deliveries
- Pending Goods Inspections
- Rejected Goods
- Goods on Hold
- Supplier Return Status
- Receipt Variance Summary

---

## 26.6 Inventory Read Models

Examples include:

- Stock Availability
- Stock by Warehouse
- Stock by Campus
- Reserved Stock
- Low Stock
- Out-of-Stock Items
- Near-Expiry Stock
- Expired Stock
- Batch Availability
- Serial Availability
- Inventory Movement History

---

## 26.7 Distribution Read Models

Examples include:

- Pending Material Requests
- Pending Stock Issues
- Active Reservations
- Stock Transfers in Transit
- Department Consumption Summary
- Inter-Campus Movement Status

---

## 26.8 Asset Read Models

Examples include:

- Asset Register
- Assets by Campus
- Assets by Department
- Assets by Custodian
- Unassigned Assets
- Assets under Warranty
- Assets with Expiring AMC
- Assets by Condition
- Assets Proposed for Retirement
- Asset Movement History

---

## 26.9 Verification Read Models

Examples include:

- Active Verification Plans
- Verification Progress
- Stock Variances
- Missing Assets
- Unverified Assets
- Reconciliation Pending
- Verification History

---

## 26.10 Disposal Read Models

Examples include:

- Disposal Candidates
- Disposal Awaiting Approval
- Retired Assets Awaiting Disposal
- Scrap Pending Handover
- Active Auctions
- Completed Disposal Cases
- Disposal History

---

## 26.11 Enterprise Resource Read Models

Multi-domain governance may use read models such as:

```text
Enterprise Resource Health
        ¦
        +-- Procurement Backlog
        +-- Supplier Performance
        +-- Inventory Availability
        +-- Critical Stock
        +-- Asset Utilisation
        +-- Verification Variances
        +-- Disposal Backlog
```

These read models shall not become canonical owners of the underlying information.
# 27. Domain Event Catalogue

## 27.1 Purpose

Domain events represent immutable business facts produced after successful state transitions within Section J.

Events enable Section J bounded contexts and external SutraERP domains to collaborate without direct database coupling or multi-domain aggregate modification.

Every event shall identify, where applicable:

- Event ID
- Event Type
- Event Version
- Occurred At
- Institution ID
- Campus ID
- Aggregate ID
- Correlation ID
- Causation ID
- Initiating Context
- Relevant Business References

Events shall describe facts that have already occurred.

---

## 27.2 Event Design Principles

Section J events shall:

- Use past-tense business terminology.
- Have one canonical publisher.
- Be immutable after publication.
- Avoid exposing internal persistence structures.
- Carry sufficient identifiers for consumers to retrieve additional information.
- Avoid embedding unnecessary sensitive information.
- Support schema versioning.
- Support idempotent consumption.
- Preserve institution and campus context where applicable.

A domain event shall not be used as a disguised command.

For example:

```text
Correct:

GoodsReceived
StockIssued
AssetAssigned

Incorrect:

PleaseReceiveGoods
IssueThisStock
AssignThisAsset
```

---

## 27.3 J01 — Procurement Planning & Sourcing Events

| Event | Meaning |
|---|---|
| ProcurementPlanCreated | Procurement plan established |
| ProcurementPlanUpdated | Procurement plan changed |
| SourcingEventInitiated | Supplier sourcing process started |
| RFQIssued | Request for quotation issued |
| SupplierResponseRecorded | Supplier response received and recorded |
| SupplierResponseEvaluated | Supplier response evaluation completed |
| SupplierRecommended | Supplier recommended through sourcing |
| SourcingEventCompleted | Sourcing process completed |
| SourcingEventCancelled | Sourcing process cancelled |

---

## 27.4 J02 — Vendor & Supplier Events

| Event | Meaning |
|---|---|
| SupplierRegistered | Supplier identity created |
| SupplierQualificationRequested | Supplier submitted for qualification |
| SupplierApproved | Supplier approved for configured activities |
| SupplierSuspended | Supplier temporarily suspended |
| SupplierReinstated | Suspended supplier restored |
| SupplierBlacklisted | Supplier prohibited according to institutional policy |
| SupplierEvaluationRecorded | Supplier performance evaluation recorded |
| SupplierDeactivated | Supplier made inactive |

Supplier status events may be consumed by J01 and J04 to prevent unauthorised procurement commitments.

---

## 27.5 J03 — Purchase Requisition Events

| Event | Meaning |
|---|---|
| PurchaseRequisitionCreated | Purchase requisition created |
| PurchaseRequisitionSubmitted | Requisition submitted for processing |
| PurchaseRequisitionApproved | Requisition approved |
| PurchaseRequisitionRejected | Requisition rejected |
| PurchaseRequisitionAmended | Approved or submitted requisition amended through authorised process |
| PurchaseRequisitionCancelled | Requisition cancelled |
| PurchaseRequisitionsConsolidated | Multiple requisitions consolidated |
| BudgetValidationRequested | Financial budget validation requested |
| BudgetValidationReferenced | Finance validation result associated with requisition |
| PurchaseRequisitionReleasedForProcurement | Requisition authorised for procurement |
| PurchaseRequisitionFulfilled | Requisition fulfilment completed |
| PurchaseRequisitionClosed | Requisition lifecycle closed |

---

## 27.6 J04 — Purchase Order Events

| Event | Meaning |
|---|---|
| PurchaseOrderCreated | Purchase Order created |
| PurchaseOrderSubmittedForApproval | Purchase Order submitted |
| PurchaseOrderApproved | Purchase Order approved |
| PurchaseOrderIssued | Purchase Order formally issued |
| SupplierAcknowledgementRecorded | Supplier acknowledgement recorded |
| PurchaseOrderAmended | Purchase Order amended through controlled versioning |
| PurchaseOrderPartiallyFulfilled | Part of the ordered quantity fulfilled |
| PurchaseOrderFullyFulfilled | Ordered obligation fulfilled |
| PurchaseOrderCancelled | Purchase Order cancelled |
| PurchaseOrderClosed | Purchase Order lifecycle closed |

---

## 27.7 J05 — Goods Receipt & Inspection Events

| Event | Meaning |
|---|---|
| GoodsReceived | Supplier delivery physically received |
| GoodsReceiptVarianceRecorded | Receipt quantity/specification variance recorded |
| GoodsInspectionRequested | Formal inspection requested |
| GoodsInspectionCompleted | Inspection completed |
| GoodsAccepted | Received goods accepted |
| GoodsRejected | Received goods rejected |
| GoodsPlacedOnHold | Goods placed into controlled hold status |
| GoodsReleasedFromHold | Held goods released |
| SupplierReturnInitiated | Return to supplier initiated |
| SupplierReturnCompleted | Supplier return completed |
| GoodsReceiptClosed | Goods Receipt lifecycle closed |

---

## 27.8 J06 — Inventory & Warehouse Events

| Event | Meaning |
|---|---|
| ItemCreated | Item Master entry created |
| ItemActivated | Item enabled for institutional use |
| ItemDeactivated | Item disabled for new transactions |
| WarehouseCreated | Warehouse/store created |
| StorageLocationCreated | Storage location established |
| InventoryReceived | Accepted goods posted into inventory |
| StockLedgerMovementPosted | Authorised inventory movement recorded |
| InventoryHoldPlaced | Inventory placed on hold |
| InventoryHoldReleased | Inventory hold removed |
| StockLevelChanged | Derived stock position materially changed |
| ReorderThresholdReached | Configured reorder condition reached |
| StockBecameUnavailable | Available stock reached configured unavailable condition |
| InventoryExpiryApproaching | Configured expiry threshold reached |
| InventoryExpired | Inventory reached expiry state |

`StockLevelChanged` shall not replace the immutable Stock Ledger.

It is an integration/event notification derived from authoritative ledger movements.

---

## 27.9 J07 — Stock Operations & Distribution Events

| Event | Meaning |
|---|---|
| MaterialRequestCreated | Internal request for existing stock created |
| StockReserved | Stock reserved for approved requirement |
| StockReservationReleased | Reservation released |
| StockIssued | Stock physically issued |
| StockReturned | Previously issued stock returned |
| StockTransferRequested | Transfer requested |
| StockTransferApproved | Transfer approved |
| StockTransferDispatched | Stock dispatched from source |
| StockTransferReceived | Stock received at destination |
| StockTransferCompleted | Transfer lifecycle completed |
| ConsumptionRecorded | Consumption acknowledgement recorded |
| StockAdjustmentRequested | Controlled stock adjustment requested |

---

## 27.10 J08 — Asset Lifecycle Events

| Event | Meaning |
|---|---|
| AssetRegistered | Operational asset created |
| AssetTagged | Asset tag assigned |
| AssetAssigned | Asset assigned to authorised custodian/unit |
| AssetReturned | Assigned asset returned |
| AssetTransferred | Asset transferred |
| AssetConditionChanged | Operational condition changed |
| AssetLocationChanged | Asset location changed |
| AssetWarrantyUpdated | Warranty reference changed |
| AssetAMCUpdated | AMC reference changed |
| AssetRetirementProposed | Asset proposed for retirement |
| AssetRetired | Asset removed from active operational use |

---

## 27.11 J09 — Verification Events

| Event | Meaning |
|---|---|
| VerificationPlanCreated | Physical verification plan created |
| VerificationStarted | Verification execution started |
| PhysicalCountRecorded | Physical inventory count recorded |
| AssetVerificationRecorded | Asset verification result recorded |
| VerificationVarianceDetected | Recorded state differs from physical state |
| VerificationVarianceInvestigated | Variance investigation completed |
| ReconciliationRecommended | Reconciliation action recommended |
| ReconciliationApproved | Reconciliation approved |
| VerificationCompleted | Verification process completed |
| VerificationClosed | Verification lifecycle formally closed |

---

## 27.12 J10 — Disposal Events

| Event | Meaning |
|---|---|
| DisposalCaseCreated | Disposal case created |
| DisposalCandidateAssessed | Resource assessed for disposal |
| DisposalRecommended | Disposal formally recommended |
| DisposalApproved | Disposal authorised |
| DisposalMethodSelected | Approved disposal method selected |
| AuctionOutcomeRecorded | Auction outcome recorded |
| ScrapHandoverRecorded | Scrap handover recorded |
| AssetDisposalExecuted | Asset physically disposed |
| InventoryDisposalExecuted | Inventory physically disposed |
| DisposalVerified | Disposal execution independently verified |
| DisposalCaseClosed | Disposal lifecycle closed |

---

# 28. Consumed Event Catalogue

## 28.1 Purpose

Section J consumes events from other SutraERP domains when those events create a legitimate resource-management consequence.

Consumed events shall not automatically trigger irreversible resource transactions unless configured business rules and required approvals have been satisfied.

---

## 28.2 Institution Foundation Events

Section J may consume events such as:

- InstitutionCreated
- CampusCreated
- CampusDeactivated
- DepartmentCreated
- DepartmentUpdated
- DepartmentDeactivated
- OrganisationalUnitChanged

Possible consequences include:

- Warehouse configuration eligibility
- Resource ownership/reference updates
- Campus resource structures
- Department allocation updates

Institutional master data remains externally owned.

---

## 28.3 Human Resources Events

Section J may consume:

- EmployeeOnboarded
- EmployeeTransferred
- EmployeeDepartmentChanged
- EmployeeSeparationInitiated
- EmployeeSeparated

Possible reactions include:

- Resource requirement recommendation
- Assigned asset review
- Asset return request
- Custody reassignment review

No HR lifecycle transition shall be automatically reversed by J.

---

## 28.4 Finance Events

Section J may consume events representing:

- BudgetValidationCompleted
- BudgetCommitmentCreated
- BudgetCommitmentReleased
- SupplierFinancialHoldApplied
- SupplierFinancialHoldReleased
- SupplierPaymentCompleted
- AssetCapitalizationCompleted
- AssetAccountingClosed

Exact event names shall conform to the canonical Section F contracts when those contracts are defined.

J00 defines the required business semantics, not ownership of Finance event naming.

---

## 28.5 Campus Operations Events

Applicable G domains may publish resource-demand or operational events.

Examples include:

- MaintenanceMaterialRequired
- LaboratoryMaterialRequired
- HostelResourceRequired
- TransportMaterialRequired
- MedicalSupplyRequired
- SecurityEquipmentRequired
- HousekeepingMaterialRequired

Section J shall not assume every operational event creates a Purchase Requisition.

The preferred decision path is:

```text
Operational Requirement
        ¦
        ?
Check Existing Resource Availability
        ¦
      +---+
      ¦   ¦
Available Unavailable
      ¦   ¦
      ?   ?
J07 Issue J03 Procurement Demand
```

---

## 28.6 Academic Events

Section J may consume approved resource requirements generated by academic operations.

Examples may include:

- TeachingResourceRequired
- LaboratoryResourceRequirementApproved
- AcademicEquipmentRequired

Academic ownership of teaching and curriculum decisions remains unchanged.

---

## 28.7 Examination Events

Section J may consume:

- ExaminationMaterialRequirementApproved
- ExaminationMaterialReturnRequested
- ExaminationMaterialReconciliationRequested

Sensitive examination resources may invoke stricter authorisation and custody rules.

---

# 29. Multi-Domain Event Matrix

## 29.1 Internal Section J Event Flow

| Publisher | Event | Typical Consumer |
|---|---|---|
| J02 | SupplierApproved | J01, J04 |
| J02 | SupplierSuspended | J01, J04 |
| J03 | PurchaseRequisitionReleasedForProcurement | J01 / J04 |
| J01 | SupplierRecommended | J04 |
| J04 | PurchaseOrderIssued | J05, Finance |
| J05 | GoodsReceived | J04 |
| J05 | GoodsAccepted | J06, J08, Finance |
| J05 | GoodsRejected | J04, J02 |
| J06 | ReorderThresholdReached | J03 |
| J07 | StockIssued | J06, Requesting Domain |
| J07 | StockReturned | J06 |
| J07 | StockTransferDispatched | J06 |
| J07 | StockTransferReceived | J06 |
| J08 | AssetRegistered | Finance |
| J08 | AssetAssigned | Section E / Requesting Domain where applicable |
| J09 | ReconciliationApproved | J06 / J08 |
| J10 | AssetDisposalExecuted | J08, Finance |
| J10 | InventoryDisposalExecuted | J06, Finance |

---

## 29.2 Financial Integration Events

Typical Section J events relevant to Finance include:

```text
PurchaseRequisitionApproved
        ¦
        ?
Budget / Commitment Processing

PurchaseOrderIssued
        ¦
        ?
Financial Commitment Reference

GoodsAccepted
        ¦
        ?
Payables / Matching Eligibility

AssetRegistered
        ¦
        ?
Capitalisation Processing

AssetDisposalExecuted
        ¦
        ?
Financial Write-Off / Disposal Accounting
```

The exact accounting response remains owned by Finance.

---

## 29.3 Requesting Domain Fulfilment Events

A requesting domain may consume:

- MaterialRequestCreated
- StockReserved
- StockIssued
- StockBecameUnavailable
- PurchaseRequisitionCreated
- PurchaseOrderIssued
- GoodsAccepted
- AssetAssigned
- ResourceDisposalCompleted where relevant

This allows operational domains to maintain purpose-specific read models without owning Section J state.

---

# 30. Business Rules & Invariants

## 30.1 Enterprise Invariants

The following rules apply across Section J.

### BR-J00-001 — Single Supplier Identity

A supplier shall have one canonical J02 identity within the applicable institutional scope.

---

### BR-J00-002 — Single Item Identity

A stock-controlled institutional item shall have one canonical J06 Item Master identity within the configured enterprise scope.

---

### BR-J00-003 — No Direct Stock Balance Editing

Authoritative stock balances shall change only through valid Stock Ledger movements.

---

### BR-J00-004 — Accepted Goods Only

Rejected goods shall not become unrestricted available stock.

---

### BR-J00-005 — Traceable Stock Movement

Every stock movement shall identify:

- Item
- Quantity
- Source where applicable
- Destination where applicable
- Business reason
- Transaction reference
- Timestamp
- Authorised actor

---

### BR-J00-006 — Reservation Does Not Equal Issue

Reserved stock remains physically held until an authorised issue or transfer occurs.

---

### BR-J00-007 — No Negative Stock by Default

Section J shall reject transactions that create negative physical stock unless an institution explicitly enables controlled negative-stock behaviour.

Negative-stock usage shall be auditable.

---

### BR-J00-008 — Supplier Eligibility

Purchase Orders shall not be issued to suppliers prohibited by applicable supplier-status rules.

Emergency exceptions, where legally and institutionally permitted, require explicit override authority and audit.

---

### BR-J00-009 — Purchase Order Traceability

Every Purchase Order shall retain its approved business authority and procurement source.

---

### BR-J00-010 — Goods Receipt Traceability

Every procurement-related Goods Receipt shall reference the relevant Purchase Order or an explicitly authorised non-PO receipt mechanism.

---

### BR-J00-011 — Asset Identity

Every individually tracked operational asset shall have one immutable Asset ID.

Asset tags may be replaced according to controlled procedures without changing Asset ID.

---

### BR-J00-012 — Asset Custody History

Asset reassignment shall never erase previous custody history.

---

### BR-J00-013 — Verification Does Not Silently Correct

Physical verification shall record discrepancies before any canonical stock or asset correction occurs.

---

### BR-J00-014 — Disposal Requires Authority

Inventory or assets requiring controlled disposal shall not be removed from canonical records merely by deletion.

---

### BR-J00-015 — Physical and Financial Separation

Section J shall not directly modify authoritative Finance ledgers, budgets, payments, book values, or depreciation records.

---

### BR-J00-016 — Multi-Domain Reference Integrity

Section J shall reference externally owned entities through stable identifiers and contracts.

It shall not become their secondary master-data owner.

---

### BR-J00-017 — Historical Business Documents

Issued Purchase Orders, completed Goods Receipts, stock movements, verification records, and disposal evidence shall preserve historical business truth.

---

### BR-J00-018 — Compensating Corrections

Completed resource transactions shall be corrected through authorised reversal, adjustment, return, or compensating transactions rather than destructive editing.

---

### BR-J00-019 — Institution Isolation

Resource data belonging to one institution shall not be visible or transactable by another institution except through explicitly authorised enterprise arrangements.

---

### BR-J00-020 — Controlled Resource Policy

Resources classified as controlled shall follow their configured additional custody, storage, issue, verification, and disposal requirements.

---

# 31. Authorization & Segregation of Duties

## 31.1 Purpose

Enterprise Resource Management involves institutional expenditure, physical custody, and resource movement.

Authorization shall therefore support both Role-Based Access Control and context-aware business restrictions.

Authentication infrastructure remains external.

Section J defines resource-management authorization semantics.

---

## 31.2 Typical Roles

Institutions may configure roles such as:

- Resource Management Administrator
- Procurement Officer
- Procurement Approver
- Supplier Administrator
- Purchase Requisition Requester
- Purchase Requisition Approver
- Purchase Order Officer
- Goods Receiver
- Goods Inspector
- Store Officer
- Warehouse Manager
- Material Requester
- Stock Issue Officer
- Asset Administrator
- Asset Custodian
- Verification Officer
- Verification Approver
- Disposal Officer
- Disposal Approver
- Auditor
- Read-Only Reviewer

Roles are illustrative and shall not be hard-coded.

---

## 31.3 Authorization Dimensions

Authorization may consider:

- Institution
- Campus
- Department
- Warehouse
- Item Category
- Asset Category
- Procurement Value Band
- Transaction Type
- Supplier Category
- Controlled Resource Classification
- Delegated Authority
- Effective Date

---

## 31.4 Segregation of Duties

The architecture shall support configurable separation such as:

```text
Requester
    ?
Approver

Purchase Order Creator
    ?
Purchase Order Approver

Goods Receiver
    ?
Goods Inspector

Store Custodian
    ?
Physical Verifier

Disposal Proposer
    ?
Disposal Approver
```

Small institutions may configure simpler responsibility models.

Auditability remains mandatory.

---

## 31.5 Value-Based Approval

Procurement approval may vary according to transaction value.

Conceptually:

```text
Purchase Value
      ¦
      ?
Configured Authority Matrix
      ¦
      +-- Level 1
      +-- Level 2
      +-- Committee
      +-- Executive Authority
```

Section J shall not hard-code monetary thresholds.

---

## 31.6 Delegation

Approvals may support:

- Temporary Delegation
- Acting Authority
- Effective Start Date
- Effective End Date
- Delegation Scope
- Automatic Expiry

Delegation shall remain auditable.

---

## 31.7 Controlled Resource Authorization

Controlled inventory may require stricter authorization for:

- Viewing Availability
- Reservation
- Issue
- Transfer
- Adjustment
- Verification
- Disposal

Examples may include sensitive examination material or institution-designated controlled resources.

---

# 32. Configuration Architecture

## 32.1 Purpose

Section J shall support institutions ranging from small colleges with largely manual stores processes to large multi-campus universities with formal procurement and warehousing.

Complex capabilities shall therefore be configurable rather than mandatory.

---

## 32.2 Procurement Configuration

Institutions may configure:

- Procurement Methods
- Approval Chains
- Value Thresholds
- Quotation Requirements
- Tender Requirements
- Emergency Procurement
- Sourcing Evaluation Criteria
- Requisition Consolidation
- Purchase Order Approval

---

## 32.3 Supplier Configuration

Institutions may configure:

- Supplier Categories
- Qualification Requirements
- Compliance Documents
- Approval Rules
- Evaluation Criteria
- Suspension Rules
- Blacklisting Rules
- Performance Thresholds

---

## 32.4 Inventory Configuration

Institutions may configure:

- Item Categories
- Units of Measure
- Warehouse Types
- Storage Hierarchy
- Batch Tracking
- Serial Tracking
- Expiry Tracking
- Reorder Rules
- Safety Stock
- Negative Stock Policy
- Reservation Policy
- Controlled Resource Policy

---

## 32.5 Asset Configuration

Institutions may configure:

- Asset Categories
- Asset Tagging Rules
- Assignment Rules
- Custody Rules
- Condition Codes
- Warranty Tracking
- AMC Tracking
- Verification Frequency
- Retirement Rules

---

## 32.6 Verification Configuration

Institutions may configure:

- Verification Types
- Verification Frequency
- Count Methods
- Stock Freeze Policy
- Variance Thresholds
- Reconciliation Approval
- Verification Independence Rules

---

## 32.7 Disposal Configuration

Institutions may configure:

- Disposal Categories
- Disposal Methods
- Approval Matrix
- Auction Requirements
- Scrap Procedures
- Verification Requirements
- Financial Notification Rules

---

## 32.8 Effective-Dated Configuration

Material governance configuration shall support:

- Effective From
- Effective Until
- Version
- Approval
- Change Reason
- Previous Version Reference

Historical transactions shall remain interpretable according to the configuration applicable when they occurred.

---

# 33. Audit & Traceability

## 33.1 Audit Principle

Section J shall maintain complete business traceability from resource requirement through final disposition.

A resource transaction shall be reconstructable without relying on mutable current-state records alone.

---

## 33.2 Procurement Traceability

The architecture should permit:

```text
Requirement
    ?
Purchase Requisition
    ?
Approval
    ?
Sourcing
    ?
Supplier Selection
    ?
Purchase Order
    ?
Goods Receipt
    ?
Inspection
```

---

## 33.3 Inventory Traceability

For a stock-controlled item:

```text
Goods Receipt
     ?
Inventory Receipt
     ?
Stock Ledger
     ?
Reservation
     ?
Issue
     ?
Return / Consumption
     ?
Verification
     ?
Disposal where applicable
```

---

## 33.4 Asset Traceability

For an asset:

```text
Acquisition
    ?
Goods Receipt
    ?
Asset Registration
    ?
Assignment
    ?
Transfer
    ?
Condition History
    ?
Verification
    ?
Retirement
    ?
Disposal
```

---

## 33.5 Audit Record

Audit information shall include, where applicable:

- Actor
- Role
- Timestamp
- Institution
- Campus
- Business Action
- Aggregate
- Previous State
- New State
- Reason
- Approval Reference
- Correlation ID
- Source Channel

---

## 33.6 Immutable History

The following shall not be destructively rewritten after finalisation:

- Approved Requisitions
- Issued Purchase Orders
- Goods Receipt Evidence
- Inspection Decisions
- Stock Ledger Entries
- Completed Stock Transfers
- Asset Custody History
- Verification Results
- Disposal Evidence

Corrections shall remain visible as subsequent business transactions.
# 34. Financial Integration

## 34.1 Purpose

Section J manages the physical and operational resource lifecycle.

Section F remains the canonical owner of financial truth.

The integration between Sections J and F shall allow procurement, receipt, inventory, assets, and disposal activities to produce appropriate financial consequences without transferring accounting ownership to Section J.

The fundamental boundary is:

```text
Section J
Physical / Operational Truth
        ¦
        ¦ Business Facts
        ?
Section F
Financial Truth
```

Section J shall never directly manipulate Finance-owned ledgers or balances.

---

## 34.2 Finance-Owned Responsibilities

Finance remains responsible for applicable:

- Budget Ownership
- Budget Availability
- Budget Commitment
- General Ledger
- Accounts Payable
- Supplier Invoice Accounting
- Payment Processing
- Tax Accounting
- Financial Asset Accounting
- Capitalisation
- Depreciation
- Book Value
- Financial Write-Off
- Financial Reporting

Section J may reference the resulting financial status but shall not become its canonical owner.

---

## 34.3 Section J Financial Facts

Section J may provide Finance with business facts including:

- Purchase Requisition Approved
- Purchase Order Issued
- Purchase Order Amended
- Goods Received
- Goods Accepted
- Goods Rejected
- Supplier Return Completed
- Inventory Receipt Posted
- Stock Adjustment Approved
- Asset Registered
- Asset Retired
- Asset Disposal Executed
- Inventory Disposal Executed

Finance determines whether and how those facts affect accounting.

---

## 34.4 Budget Validation

Where budget control is enabled:

```text
J03 Purchase Requisition
        ¦
        ¦ Budget Validation Request
        ?
F — Finance
        ¦
        +-- Available
        +-- Insufficient
        +-- Conditional
        +-- Not Applicable
        ¦
        ?
J03 Requisition Decision
```

Section J stores:

- Validation Reference
- Validation Status
- Validation Timestamp
- Relevant Budget Reference where permitted

Section J shall not maintain a competing authoritative budget balance.

---

## 34.5 Budget Commitment

Where Finance supports commitment accounting, a procurement lifecycle may result in:

```text
Purchase Requisition
        ¦
        ?
Budget Validation
        ¦
        ?
Purchase Order Issued
        ¦
        ?
Financial Commitment
        ¦
        ?
Goods Accepted
        ¦
        ?
Invoice / Liability Processing
        ¦
        ?
Commitment Adjustment / Release
```

The financial commitment remains Finance-owned.

---

## 34.6 Three-Way Matching Boundary

Where applicable, Finance may perform or coordinate matching between:

```text
Purchase Order
      +
Goods Receipt / Acceptance
      +
Supplier Invoice
      =
Payment Eligibility
```

Canonical ownership remains:

| Record | Owner |
|---|---|
| Purchase Order | J04 |
| Goods Receipt / Acceptance | J05 |
| Supplier Invoice | Finance |
| Payment | Finance |

Section J shall provide immutable references required for matching.

---

## 34.7 Inventory Valuation Boundary

J06 owns:

- Physical Quantity
- Stock Location
- Batch / Serial
- Stock Movement
- Stock Ledger

Finance owns authoritative accounting valuation where financial inventory valuation is required.

Section J may provide physical movement facts and configured operational cost references.

It shall not independently establish competing financial books.

---

## 34.8 Asset Capitalisation

An operational asset may require financial capitalisation.

```text
J05 Goods Accepted
        ¦
        ?
J08 Asset Registered
        ¦
        ¦ AssetRegistered
        ?
Finance
        ¦
        ?
Capitalisation
        ¦
        ¦ Financial Reference
        ?
J08 Read Model / Reference
```

J08 may display:

- Capitalisation Status
- Finance Asset Reference
- Financial Status Summary

Authoritative financial values remain in Finance.

---

## 34.9 Depreciation

J08 shall not calculate authoritative accounting depreciation.

Finance owns:

- Depreciation Method
- Useful Life for Accounting
- Depreciation Schedule
- Accumulated Depreciation
- Net Book Value
- Depreciation Journal

Operational useful-life or condition information may exist in J08 for maintenance and retirement decisions without becoming accounting depreciation.

---

## 34.10 Disposal Financial Integration

```text
J10 Disposal Executed
        ¦
        ?
Finance Notification
        ¦
        ?
Financial Assessment
        ¦
        +-- Write-Off
        +-- Sale Proceeds
        +-- Gain / Loss
        +-- Asset Accounting Closure
```

Physical disposal shall not directly manipulate financial balances.

---

# 35. Campus Operations Integration

## 35.1 Purpose

Section G is one of the primary consumers of Enterprise Resource Management.

Section G owns campus operational activities.

Section J owns the physical-resource capabilities supporting those activities.

The boundary shall be:

```text
Section G
Operational Need
      ¦
      ?
Section J
Resource Fulfilment
      ¦
      ?
Section G
Operational Consumption
```

---

## 35.2 Facilities Management

Facilities may require:

- Electrical Spares
- Plumbing Materials
- Hardware
- Tools
- Replacement Components
- Maintenance Consumables

Typical interaction:

```text
G01 Maintenance Requirement
          ¦
          ?
J07 Material Request
          ¦
      +--------+
      ¦        ¦
 Available   Unavailable
      ¦        ¦
      ?        ?
Stock Issue   J03 Purchase Requisition
```

G01 owns maintenance execution.

J owns material fulfilment.

---

## 35.3 Laboratory Operations

Laboratories may require:

- Chemicals
- Glassware
- Consumables
- Instruments
- Equipment
- Replacement Parts

J06 owns physical stock.

The laboratory domain may own specialist information such as:

- Chemical Usage Rules
- Academic Experiment Context
- Laboratory Safety Requirements
- Experiment Consumption Context

J06 shall not become a laboratory information system.

---

## 35.4 Hostel Operations

Hostel resource requirements may include:

- Mattresses
- Furniture
- Linen
- Cleaning Materials
- Maintenance Supplies
- Appliances

Section G owns hostel operations.

Section J owns applicable procurement, inventory, issue, asset registration, and disposal.

---

## 35.5 Transport Operations

Transport resource requirements may include:

- Spare Parts
- Tyres
- Lubricants
- Maintenance Materials
- Vehicle Equipment
- Operational Consumables

Transport owns:

- Vehicle Operations
- Route Operations
- Trip Operations
- Driver Operations
- Operational Maintenance Requirements

J owns applicable stock and resource lifecycle.

Fuel-management depth shall remain configurable according to institutional requirements and existing Section G boundaries.

---

## 35.6 Health Services

Institutional health services may require:

- Medical Consumables
- First-Aid Supplies
- Medicines where institutionally managed
- Medical Equipment

J06 may support:

- Batch Tracking
- Expiry Tracking
- Controlled Stock
- Stock Issue

The health domain retains ownership of:

- Clinical Use
- Patient Context
- Medical Decisions
- Dispensing Rules where clinically applicable

Section J shall not become a clinical or pharmacy decision system.

---

## 35.7 Security Operations

Security resource requirements may include:

- Communication Equipment
- Uniform-Related Supplies
- Safety Equipment
- Access-Control Hardware
- Other Security Equipment

Security operations may be outsourced.

The architecture shall therefore permit institutional assets to be:

- Institution-Owned
- Vendor-Provided
- Temporarily Assigned
- Externally Maintained

Vendor-owned equipment shall not automatically become an institutional asset.

---

## 35.8 Housekeeping Operations

Housekeeping may consume:

- Cleaning Chemicals
- Cleaning Supplies
- Consumables
- Equipment
- Protective Supplies

Where housekeeping is outsourced, institutions may choose whether:

- Supplies are institution-owned and issued through J.
- Supplies are vendor-owned and outside institutional inventory.
- A hybrid model applies.

This shall remain configurable.

---

## 35.9 Library Operations

Section J may support procurement and physical receipt of library acquisitions where institutional policy routes those acquisitions through enterprise procurement.

Library-specific ownership remains with the Library domain for:

- Bibliographic Records
- Accession / Library Catalogue
- Circulation
- Borrowing
- Reservation
- Library Fines

An enterprise Item Master shall not replace the bibliographic catalogue.

---

# 36. Academic, HR, Student & Examination Integration

## 36.1 Academic Integration

Academic domains may generate requirements for:

- Teaching Materials
- Laboratory Materials
- Academic Equipment
- Classroom Equipment
- Department Resources

Academic domains own the academic justification.

Section J owns resource fulfilment.

---

## 36.2 HR Integration

HR lifecycle events may create resource requirements.

Example:

```text
Employee Onboarded
       ¦
       ?
Configured Resource Requirement
       ¦
       ?
Stock / Asset Availability
       ¦
       ?
Asset Assignment
```

Examples may include:

- Laptop
- Desktop
- Institutional Device
- Office Equipment
- Other Assigned Resource

Automatic assignment shall not be mandatory.

Institutions may continue to use manual recommendations and allocation.

---

## 36.3 Employee Transfer

Employee transfer may require review of assigned resources.

Possible outcomes include:

- Asset Retained
- Asset Returned
- Asset Transferred with Employee
- Asset Reassigned
- No Action

The policy shall remain configurable.

---

## 36.4 Employee Separation

Employee separation may trigger:

```text
Separation Initiated
       ¦
       ?
Assigned Resource Review
       ¦
       ?
Outstanding Assets Identified
       ¦
       ?
Return / Exception / Approval
```

Section J may publish resource-clearance status.

HR remains owner of the separation lifecycle.

---

## 36.5 Student Resource Integration

Where institutions issue physical resources to students, Section J may support:

- Resource Issue
- Temporary Custody
- Return
- Loss / Damage Reference

Examples may include:

- Institutional Equipment
- Kits
- Uniform-Related Items
- Department Equipment

Student identity remains owned by the Student domain.

Library circulation shall remain within Library Management rather than being replaced by generic J07 stock issue.

---

## 36.6 Examination Integration

Examination Management may request controlled materials.

Example:

```text
I — Examination
Material Requirement
        ¦
        ?
J — Resource Fulfilment
        ¦
        ?
Controlled Issue
        ¦
        ?
I — Examination Usage
        ¦
        ?
Return / Reconciliation
```

Applicable materials may include:

- Answer Books
- Stationery
- Packaging Materials
- Seals
- Other Controlled Physical Materials

---

## 36.7 Examination Confidentiality

Examination-controlled resources may require:

- Restricted Visibility
- Restricted Warehouse Access
- Named Custodian
- Dual Authorization
- Controlled Issue
- Chain of Custody
- Mandatory Reconciliation

The exact controls shall be determined by examination policy.

Section J provides the physical resource-control mechanism.

Section I owns examination confidentiality policy and examination usage.

---

# 37. Exception & Reconciliation Architecture

## 37.1 Purpose

Resource-management systems must represent operational exceptions explicitly.

Exceptions shall not be resolved through hidden data editing.

---

## 37.2 Procurement Exceptions

Examples include:

- Budget Unavailable
- Approval Delayed
- Supplier Unavailable
- No Valid Quotation
- Supplier Suspended
- Procurement Cancelled
- Emergency Requirement

Exceptions shall retain reason, actor, decision, and resolution.

---

## 37.3 Purchase Order Exceptions

Examples include:

- Supplier Rejection
- Delivery Delay
- Partial Delivery
- Order Amendment
- Quantity Change
- Cancellation
- Supplier Substitution through authorised process

---

## 37.4 Goods Receipt Exceptions

Examples include:

- Short Receipt
- Excess Receipt
- Damaged Goods
- Incorrect Goods
- Failed Inspection
- Missing Documentation
- Unauthorised Delivery
- Expired / Near-Expiry Delivery

Each exception shall result in an explicit disposition.

---

## 37.5 Inventory Exceptions

Examples include:

- Insufficient Stock
- Stock Mismatch
- Damaged Stock
- Expired Stock
- Missing Stock
- Incorrect Batch
- Incorrect Serial
- Transfer Difference
- Unreceived Transfer

---

## 37.6 Asset Exceptions

Examples include:

- Missing Asset
- Damaged Asset
- Custody Mismatch
- Location Mismatch
- Unauthorised Transfer
- Asset Not Returned
- Unserviceable Asset

---

## 37.7 Reconciliation Principle

Reconciliation shall follow:

```text
Exception
    ¦
    ?
Record
    ¦
    ?
Investigate
    ¦
    ?
Decision
    ¦
    ?
Approval where required
    ¦
    ?
Corrective Transaction
    ¦
    ?
Closure
```

Deletion of evidence shall not constitute reconciliation.

---

## 37.8 Multi-Domain Reconciliation

Some exceptions require multiple domains.

Example:

```text
Physical Stock Difference
        ¦
        ?
J09 Verification
        ¦
        ?
J06 Adjustment
        ¦
        ?
Finance
Financial Consequence
```

Each domain owns its own correction.

A distributed transaction across J06 and Finance shall not be required.

---

# 38. Reporting & KPI Responsibilities

## 38.1 Purpose

Section J shall provide operational and governance information necessary to understand institutional resource performance.

Reporting shall use read models rather than transactional aggregates for broad analytics.

---

## 38.2 Procurement KPIs

Examples include:

- Procurement Cycle Time
- Requisition Approval Time
- Sourcing Cycle Time
- Purchase Order Cycle Time
- Emergency Procurement Rate
- Requisition-to-PO Conversion
- Procurement Backlog

---

## 38.3 Supplier KPIs

Examples include:

- On-Time Delivery Rate
- Goods Acceptance Rate
- Rejection Rate
- Supplier Response Rate
- Order Fulfilment Rate
- Supplier Performance Score

---

## 38.4 Inventory KPIs

Examples include:

- Inventory Accuracy
- Stock Availability
- Stock-Out Frequency
- Excess Stock
- Slow-Moving Stock
- Near-Expiry Stock
- Expired Stock
- Inventory Adjustment Frequency
- Inventory Turnover where financially supported

---

## 38.5 Warehouse KPIs

Examples include:

- Receipt Processing Time
- Issue Processing Time
- Transfer Processing Time
- Pending Material Requests
- Pending Receipts
- Stock Variance
- Warehouse Utilisation where location-capacity data exists

---

## 38.6 Asset KPIs

Examples include:

- Total Active Assets
- Assigned Assets
- Unassigned Assets
- Assets by Condition
- Asset Transfer Frequency
- Assets Pending Verification
- Assets Pending Retirement
- Assets Pending Disposal

Financial asset-value KPIs remain Finance-owned.

---

## 38.7 Verification KPIs

Examples include:

- Verification Completion Rate
- Stock Accuracy Rate
- Asset Verification Rate
- Variance Rate
- Missing Asset Count
- Reconciliation Cycle Time
- Outstanding Variances

---

## 38.8 Disposal KPIs

Examples include:

- Disposal Backlog
- Retirement-to-Disposal Cycle Time
- Scrap Pending Handover
- Disposal Cases Pending Approval
- Completed Disposal Cases

Financial gains or losses remain Finance reporting responsibilities.

---

## 38.9 Enterprise Resource Dashboard

An enterprise resource dashboard may combine read models such as:

```text
Enterprise Resource Management

Procurement
+-- Open Requisitions
+-- Pending Approvals
+-- Open Purchase Orders

Inventory
+-- Critical Stock
+-- Low Stock
+-- Near Expiry
+-- Transfers in Transit

Assets
+-- Active Assets
+-- Unassigned Assets
+-- Verification Due
+-- Retirement Pending

Governance
+-- Verification Variances
+-- Disposal Backlog
+-- Supplier Exceptions
```

Dashboard presentation may be provided through the enterprise dashboard capability.

Section J remains owner of the underlying resource metrics.

---

# 39. Future Extension Points

## 39.1 Principle

Future technologies shall enhance Section J without changing its canonical ownership model.

---

## 39.2 Barcode

Future support may include:

- Item Barcode
- Asset Barcode
- Goods Receipt Scanning
- Stock Issue Scanning
- Physical Verification Scanning

Manual entry shall remain supported.

---

## 39.3 QR Codes

QR-based asset identification may support:

- Asset Lookup
- Assignment
- Transfer
- Verification
- Maintenance Reference

QR codes shall represent identifiers, not become the canonical asset record.

---

## 39.4 RFID

Potential future capabilities include:

- RFID Asset Tracking
- RFID Warehouse Counting
- Automated Verification
- Controlled Resource Movement Detection

RFID shall remain optional.

---

## 39.5 IoT

Potential integrations may include:

- Warehouse Environmental Monitoring
- Cold Storage Monitoring
- Equipment Telemetry
- Automated Resource Status

IoT observations shall not directly rewrite canonical inventory without validated business processing.

---

## 39.6 AI-Assisted Procurement

Future AI capabilities may support:

- Demand Forecasting
- Reorder Recommendation
- Supplier Recommendation
- Procurement Anomaly Detection
- Price Trend Analysis
- Stock Optimisation
- Disposal Candidate Recommendation

AI output shall be advisory by default.

Human approval shall remain available for consequential procurement and disposal decisions.

---

## 39.7 Supplier Portal

A future supplier portal may support:

- Supplier Registration
- Document Submission
- RFQ Response
- Purchase Order Acknowledgement
- Delivery Information
- Performance Visibility

External supplier access shall remain constrained to authorised supplier-specific information.

---

## 39.8 Mobile Stores Operations

Future mobile workflows may support:

- Goods Receipt
- Stock Issue
- Transfer
- Asset Assignment
- Verification
- Disposal Evidence

Mobile interfaces shall consume the same canonical application contracts.

---

# 40. Section-Level Architectural Constraints

The following constraints are mandatory for all J01—J10 designs.

### AC-J00-001

No J-domain may create an alternative canonical Supplier Master outside J02.

### AC-J00-002

No J-domain or external domain may create an alternative canonical Item Master for enterprise inventory outside J06.

Specialist domain catalogues may reference J06 items.

### AC-J00-003

Stock quantity shall be derived from authorised inventory movements.

### AC-J00-004

J07 shall not independently own canonical stock balance.

### AC-J00-005

J08 shall not own authoritative accounting depreciation.

### AC-J00-006

Finance shall not become the operational Asset Register owner.

### AC-J00-007

J09 verification shall not silently modify J06 or J08 state.

### AC-J00-008

J10 disposal shall not directly modify Finance ledgers.

### AC-J00-009

Multi-domain integrations shall not rely on direct shared-table writes.

### AC-J00-010

Every finalised resource transaction shall remain auditable.

### AC-J00-011

Manual operational paths shall remain available where automation is optional.

### AC-J00-012

Multi-campus operation shall not require separate deployments or duplicate master data.

### AC-J00-013

Configuration shall be institution-scoped unless explicitly defined as enterprise-global.

### AC-J00-014

A requesting domain shall not gain canonical inventory ownership by consuming resources.

### AC-J00-015

Financial truth and physical-resource truth shall remain explicitly separated.

### AC-J00-016

Sensitive and controlled resources shall support additional policy-driven authorization without requiring a separate inventory architecture.

### AC-J00-017

Historical business evidence shall not be destroyed by later master-data changes.

### AC-J00-018

Multi-domain failures shall be recoverable through idempotency, retry, reconciliation, or compensating transactions rather than distributed database transactions.

---

# 41. Governance Decisions

## GD-J00-001 — Section J Is an Enterprise Section

**Decision:** Enterprise Resource Management is established as a peer Level-2 enterprise section.

**Rationale:** Procurement, inventory, warehousing, assets, verification, and disposal serve multiple SutraERP domains and therefore shall not belong exclusively to Finance or Campus Operations.

---

## GD-J00-002 — Inventory Is Not Owned by Campus Operations

**Decision:** J06 owns enterprise inventory.

**Rationale:** Campus Operations consumes inventory but does not represent the complete institutional resource lifecycle.

---

## GD-J00-003 — Procurement Is Separate from Financial Accounting

**Decision:** Section J owns procurement business processes while Finance owns accounting and payment.

**Rationale:** Purchasing and accounting are strongly integrated but represent separate business capabilities.

---

## GD-J00-004 — Operational and Financial Assets Are Separate Models

**Decision:** J08 owns operational asset lifecycle; Finance owns financial asset accounting.

**Rationale:** Custody, location, condition, assignment, and operational lifecycle are distinct from capitalization, depreciation, and book value.

---

## GD-J00-005 — One Enterprise Stock Ledger

**Decision:** J06 provides the canonical physical Stock Ledger.

**Rationale:** Independent stock ledgers in laboratories, housekeeping, medical services, facilities, examinations, or other domains would fragment enterprise inventory truth.

---

## GD-J00-006 — Specialist Semantics Remain with Specialist Domains

**Decision:** Section J owns physical-resource state but not specialist operational meaning.

**Examples:**

- Laboratory owns academic/chemical usage context.
- Health owns clinical usage context.
- Examination owns examination confidentiality and usage.
- Facilities owns maintenance purpose.

---

## GD-J00-007 — Automation Is Optional

**Decision:** Barcode, RFID, IoT, AI, and automated warehousing are extension capabilities.

**Rationale:** Institutions with manual resource-management processes must remain fully supported.

---

## GD-J00-008 — Configuration over Institutional Assumption

**Decision:** Section J shall model capabilities rather than assume a particular purchase department, store structure, approval hierarchy, or staffing model.

---

## GD-J00-009 — Verification Is Independent of Correction

**Decision:** Physical discrepancy identification and canonical correction remain separate actions.

**Rationale:** This preserves auditability and segregation of duties.

---

## GD-J00-010 — Disposal Is a Business Lifecycle

**Decision:** Disposal shall be represented explicitly rather than implemented as deletion or an asset status edit.

**Rationale:** Institutional resources require traceable end-of-life governance.
# 42. Glossary

## 42.1 Purpose

This glossary establishes canonical terminology for Section J.

J01—J10 shall use these terms consistently unless a domain document explicitly introduces a more specialized concept without changing the meaning of an existing canonical term.

---

## 42.2 Enterprise Resource Management

**Enterprise Resource Management (ERM)**

The SutraERP enterprise capability responsible for procurement, supplier management, physical inventory, warehousing, stock operations, operational asset lifecycle, physical verification, and disposal of institutional physical resources.

Enterprise Resource Management does not mean the entire SutraERP platform and shall not be confused with the generic industry term "Enterprise Resource Planning."

---

## 42.3 Resource

**Resource**

A good, material, item, equipment unit, or other physical resource acquired, held, distributed, assigned, consumed, verified, or disposed of by the institution.

A resource may be:

- Stock-Controlled
- Non-Stock
- Consumable
- Durable
- Asset-Tracked
- Controlled
- Batch-Tracked
- Serial-Tracked
- Expiry-Tracked

---

## 42.4 Item

**Item**

The canonical enterprise definition of a type of physical good managed by J06.

An Item represents a resource definition rather than a specific physical asset instance.

Example:

```text
Item
Laptop — Dell Latitude Series

          ¦
          +-- Asset Instance: LAP-0001
          +-- Asset Instance: LAP-0002
          +-- Asset Instance: LAP-0003
```

---

## 42.5 Item Master

**Item Master**

The authoritative catalogue of enterprise inventory items owned by J06.

Specialist domain catalogues may reference Item Master entries but shall not become alternative canonical inventory masters.

---

## 42.6 Supplier

**Supplier**

An external or approved supplying party from which the institution may procure goods or services.

J02 owns the canonical Supplier identity.

The terms `Supplier` and `Vendor` may appear in institutional terminology, but Section J shall use **Supplier** as the preferred canonical business term unless an existing repository-wide contract requires `Vendor`.

---

## 42.7 Supplier Master

**Supplier Master**

The authoritative collection of supplier identities, classifications, qualifications, statuses, and operational supplier information owned by J02.

Finance-specific supplier accounting information does not constitute a separate Supplier Master.

---

## 42.8 Procurement

**Procurement**

The governed process through which the institution acquires required goods or services.

Procurement may include:

```text
Requirement
   ?
Requisition
   ?
Sourcing
   ?
Supplier Selection
   ?
Purchase Order
   ?
Receipt
```

Not every procurement requires every stage.

---

## 42.9 Sourcing

**Sourcing**

The process of identifying, inviting, evaluating, and recommending suppliers for a procurement requirement.

Sourcing may use:

- RFQ
- RFP
- Tender
- Rate Contract
- Direct Procurement
- Other Configured Methods

---

## 42.10 Purchase Requisition

**Purchase Requisition (PR)**

A formal institutional request to procure goods or services that cannot or should not be fulfilled directly from existing stock.

A Purchase Requisition is not a Purchase Order.

---

## 42.11 Material Request

**Material Request**

An internal request for fulfilment from existing institutional stock.

The distinction is:

```text
Material Request
     ?
Existing Stock

Purchase Requisition
     ?
External Procurement
```

A Material Request may result in a Purchase Requisition when stock is unavailable and procurement is authorised.

---

## 42.12 Purchase Order

**Purchase Order (PO)**

An authorised institutional purchasing commitment issued to a supplier.

J04 owns the canonical Purchase Order.

---

## 42.13 Goods Receipt

**Goods Receipt**

The formal recording that physical goods have arrived at an authorised institutional receiving point.

Goods Receipt does not automatically imply quality acceptance.

---

## 42.14 Goods Receipt Note

**Goods Receipt Note (GRN)**

A business document or representation evidencing a Goods Receipt.

The architecture treats the Goods Receipt as the business concept and allows institutions to generate a GRN document where required.

---

## 42.15 Inspection

**Inspection**

The authorised assessment of received goods to determine whether they satisfy applicable quantity, condition, specification, quality, or other acceptance requirements.

---

## 42.16 Acceptance

**Acceptance**

The business decision that received goods are approved for the appropriate downstream lifecycle.

Acceptance may result in:

- Inventory Receipt
- Asset Registration
- Direct Delivery
- Direct Consumption

depending on classification.

---

## 42.17 Inventory

**Inventory**

Physical institutional resources whose quantities and movements are controlled through J06.

Inventory does not mean every physical object owned by the institution.

---

## 42.18 Warehouse

**Warehouse**

A logical or physical institutional location responsible for holding controlled inventory.

A Warehouse may represent:

- Central Store
- Campus Store
- Department Store
- Laboratory Store
- Medical Store
- Examination Store
- Other Configured Store

The term does not require the institution to operate a conventional commercial warehouse.

---

## 42.19 Storage Location

**Storage Location**

A subdivision of a Warehouse used to identify where inventory is held.

Use of Storage Locations is configurable.

---

## 42.20 Bin

**Bin**

An optional granular storage position within a Storage Location.

Bin-level inventory management shall not be mandatory.

---

## 42.21 Stock Ledger

**Stock Ledger**

The authoritative immutable sequence of stock-affecting transactions used to derive physical inventory position.

Stock balances shall not be manually overwritten independently of authorised ledger movements.

---

## 42.22 Stock Balance

**Stock Balance**

The physical quantity derived from authoritative Stock Ledger transactions for an applicable combination of item, location, batch, serial, or other tracking dimension.

---

## 42.23 Available Stock

**Available Stock**

The portion of valid physical stock currently available for new reservation or issue after applicable holds and reservations are considered.

---

## 42.24 Reserved Stock

**Reserved Stock**

Physical stock allocated to an approved requirement but not yet physically issued.

Reservation does not constitute consumption.

---

## 42.25 Stock Issue

**Stock Issue**

The authorised physical release of inventory from institutional stock to an approved recipient, unit, purpose, or operational context.

---

## 42.26 Stock Return

**Stock Return**

The controlled return of previously issued stock to institutional inventory.

Returned material may require validation before becoming available stock.

---

## 42.27 Stock Transfer

**Stock Transfer**

The controlled movement of inventory between authorised inventory locations.

Transfers may be:

- Intra-Warehouse
- Inter-Warehouse
- Inter-Campus

---

## 42.28 Consumption

**Consumption**

The business acknowledgement that an issued consumable has been used for its intended operational purpose.

Detailed consumption recording is configurable.

---

## 42.29 Stock Adjustment

**Stock Adjustment**

An authorised compensating inventory transaction used to reconcile legitimate physical differences or corrections.

A Stock Adjustment shall not be implemented as direct balance editing.

---

## 42.30 Batch / Lot

**Batch / Lot**

A collection of inventory units sharing a common production, acquisition, or traceability characteristic.

Batch tracking is optional and classification-driven.

---

## 42.31 Serial Number

**Serial Number**

An identifier associated with an individually distinguishable physical unit.

Serial tracking may be used for inventory and may later contribute to asset identification.

---

## 42.32 Asset

**Asset**

An individually tracked institutional physical resource requiring operational lifecycle management beyond ordinary inventory quantity control.

J08 owns the operational Asset.

Finance owns applicable financial asset accounting.

---

## 42.33 Operational Asset Register

**Operational Asset Register**

The authoritative J08 record of individually tracked institutional assets and their operational state.

It may include:

- Identity
- Tag
- Location
- Custody
- Assignment
- Condition
- Warranty
- Operational Status

It does not own authoritative book value or depreciation.

---

## 42.34 Asset Tag

**Asset Tag**

A human-readable, barcode, QR, RFID, or other institutional identifier physically or logically associated with an Asset.

Asset Tag and immutable Asset ID are distinct concepts.

---

## 42.35 Asset Custodian

**Asset Custodian**

The employee, department, unit, or other authorised party operationally responsible for an assigned Asset.

Custody does not imply legal ownership.

---

## 42.36 Asset Assignment

**Asset Assignment**

The controlled establishment of custody or operational responsibility for an Asset.

---

## 42.37 Asset Transfer

**Asset Transfer**

The controlled movement of an Asset between custodians, organisational units, campuses, or locations.

---

## 42.38 Asset Retirement

**Asset Retirement**

The business state in which an Asset is removed from normal active operational use.

Retirement does not automatically constitute physical disposal.

---

## 42.39 Physical Verification

**Physical Verification**

The controlled process of comparing recorded inventory or asset information against physical reality.

---

## 42.40 Verification Variance

**Verification Variance**

A documented difference between the authoritative recorded state and the observed physical state.

A variance requires investigation and shall not automatically modify canonical records.

---

## 42.41 Reconciliation

**Reconciliation**

The controlled process through which an identified discrepancy is investigated, resolved, and—where required—corrected through authorised business transactions.

---

## 42.42 Disposal

**Disposal**

The authorised physical end-of-life process through which an institutional resource permanently leaves active institutional resource custody or is otherwise formally disposed of.

---

## 42.43 Scrap

**Scrap**

A disposal classification for resources no longer suitable for their intended operational use and designated for scrap processing.

---

## 42.44 Write-Off

**Write-Off**

The financial recognition of an approved resource loss, disposal, or accounting removal.

Physical disposal is governed by J10.

Financial Write-Off is governed by Finance.

---

## 42.45 Controlled Resource

**Controlled Resource**

A resource requiring additional policy-driven restrictions beyond ordinary inventory control.

Additional controls may include:

- Restricted Visibility
- Restricted Storage
- Special Approval
- Named Custody
- Chain of Custody
- Mandatory Reconciliation
- Restricted Disposal

---

## 42.46 Non-Stock Purchase

**Non-Stock Purchase**

A procurement transaction for goods that do not require ordinary warehouse inventory lifecycle management according to institutional policy.

---

## 42.47 Service Procurement

**Service Procurement**

Procurement of non-physical services.

Service Procurement may use J01—J04 capabilities but does not generate physical inventory.

---

# 43. Canonical Ownership Matrix

## 43.1 Purpose

The following matrix is the authoritative Section J ownership contract.

J01—J10 shall conform to this matrix.

Where another SutraERP document conflicts with this matrix, the conflict shall be explicitly reconciled through architecture governance before implementation rather than silently creating duplicate ownership.

---

## 43.2 Section J Internal Ownership

| Business Capability / Concept | Canonical Owner | Explicit Non-Owners |
|---|---|---|
| Procurement Planning | J01 | J03, J04, Finance |
| Sourcing Event | J01 | J02, J03, J04 |
| RFQ / RFP / Tender Process | J01 | J02, J04 |
| Supplier Selection Recommendation | J01 | J02 |
| Supplier Master | J02 | Finance, J01, J04 |
| Supplier Qualification | J02 | J01, Finance |
| Supplier Operational Status | J02 | J04, Finance |
| Supplier Performance | J02 | J01, J04 |
| Purchase Requisition | J03 | Requesting Domain, Finance |
| Requisition Approval State | J03 | J01, J04 |
| Purchase Order | J04 | J03, Finance |
| PO Fulfilment State | J04 | J05 |
| Goods Receipt | J05 | J04, J06 |
| Goods Inspection | J05 | Requesting Domain |
| Goods Acceptance | J05 | J06, Finance |
| Item Master | J06 | G Domains, Finance |
| Warehouse Registry | J06 | Campus Operations |
| Storage Location | J06 | Requesting Domain |
| Physical Stock Ledger | J06 | Finance, G Domains |
| Physical Stock Balance | J06 | J07, Requesting Domain |
| Batch / Lot Inventory | J06 | Specialist Domain |
| Inventory Serial Tracking | J06 | J08 |
| Stock Reservation | J07 | J06 |
| Material Request | J07 | J03 |
| Stock Issue | J07 | Requesting Domain |
| Stock Return | J07 | Requesting Domain |
| Stock Transfer | J07 | J06 |
| Consumption Record | J07 | J06 |
| Operational Asset | J08 | Finance |
| Asset Tag | J08 | Finance |
| Asset Custody | J08 | HR, Requesting Domain |
| Asset Assignment | J08 | HR |
| Asset Operational Location | J08 | Facilities |
| Asset Operational Condition | J08 | Finance |
| Asset Retirement | J08 | J10, Finance |
| Verification Plan | J09 | J06, J08 |
| Physical Count | J09 | Warehouse |
| Verification Variance | J09 | J06, J08 |
| Reconciliation Recommendation | J09 | J06, J08 |
| Disposal Case | J10 | J08, Finance |
| Physical Disposal | J10 | Finance |
| Scrap Process | J10 | J06, J08 |
| Auction Disposal Process | J10 | Finance |
| Physical Disposal Evidence | J10 | Finance |

---

## 43.3 Finance Boundary Ownership

| Concept | Owner |
|---|---|
| Institutional Budget | Finance |
| Budget Balance | Finance |
| Budget Commitment | Finance |
| General Ledger | Finance |
| Accounts Payable | Finance |
| Supplier Invoice Accounting | Finance |
| Payment | Finance |
| Financial Asset Record | Finance |
| Asset Capitalisation | Finance |
| Accounting Depreciation | Finance |
| Asset Book Value | Finance |
| Financial Write-Off | Finance |
| Financial Gain / Loss on Disposal | Finance |

Section J may retain references and read-model projections of these concepts but shall not become authoritative for them.

---

## 43.4 Operational Domain Boundary

| Concept | Owner |
|---|---|
| Maintenance Purpose | Relevant Campus Operations Domain |
| Laboratory Academic Usage | Laboratory Domain |
| Hostel Operational Usage | Hostel Domain |
| Transport Operational Usage | Transport Domain |
| Clinical Usage | Health Services Domain |
| Security Operational Usage | Security Domain |
| Housekeeping Operational Usage | Housekeeping Domain |
| Library Catalogue / Circulation | Library Domain |
| Employee Lifecycle | HR |
| Academic Requirement | Academic Domain |
| Examination Usage | Examination Domain |
| Examination Confidentiality Policy | Examination Domain |

Section J owns only the applicable physical-resource fulfilment and lifecycle.

---

# 44. Dependency Matrix

## 44.1 Dependency Principles

Dependencies shall be directional and contract-based.

A dependency means that one bounded context requires a business capability, identifier, validation, or event owned by another context.

A dependency shall not imply shared database ownership.

---

## 44.2 Internal Section J Dependency Matrix

| Domain | Primary Upstream Dependencies | Primary Downstream Consumers |
|---|---|---|
| J01 Procurement Planning & Sourcing | J02, J03 | J04 |
| J02 Vendor & Supplier Management | Institution/Foundation references | J01, J04, J05 |
| J03 Purchase Requisition & Approval | Requesting Domains, Finance, J06/J07 | J01, J04 |
| J04 Purchase Order Management | J01, J02, J03, Finance validation where applicable | J05, Finance |
| J05 Goods Receipt & Inspection | J04, J02 | J06, J08, Finance |
| J06 Inventory & Warehouse Management | J05, Institution/Foundation | J07, J09, J10, Requesting Domains |
| J07 Stock Operations & Distribution | J06, Requesting Domains | J06, Requesting Domains, J09 |
| J08 Asset Lifecycle Management | J05/J06 where applicable, Institution, HR | J09, J10, Finance, Requesting Domains |
| J09 Physical Verification & Audit | J06, J07, J08 | J06, J08, J10, Finance where required |
| J10 Disposal & Write-Off | J06, J08, J09 | Finance |

---

## 44.3 Enterprise Dependency Matrix

| Section | J Depends On | Section Depends On J For |
|---|---|---|
| A — Platform/Core | Identity, authorization, configuration, workflow infrastructure | Resource-management application capabilities |
| B — Institution Foundation | Institution, campus, department and organisational references | Resource visibility and allocation |
| C — Student Lifecycle | Student identity/context where physical resources are assigned | Applicable student resource issue/return |
| D — Academic Operations | Academic resource requirements | Procurement, inventory and equipment fulfilment |
| E — Human Resources | Employee identity and lifecycle | Asset assignment, custody and return |
| F — Financial Management | Budget, accounting, AP, payments, financial asset accounting | Procurement/receipt/asset physical facts |
| G — Campus Operations | Operational resource requirements | Material, inventory and asset fulfilment |
| H — Enterprise Services | Documents, notifications, dashboards, integration services where canonically owned | Resource events and resource read models |
| I — Examination Management | Examination resource requirements and control policy | Controlled physical resource fulfilment |

---

## 44.4 Dependency Direction Rules

### Rule 1 — Finance Integration

```text
J --business facts--? F
J ?-financial status- F
```

Neither section directly modifies the other's aggregates.

---

### Rule 2 — Operational Resource Requests

```text
Operational Domain
        ¦
        ¦ Requirement
        ?
        J
        ¦
        ¦ Fulfilment Status
        ?
Operational Domain
```

The operational domain remains owner of the business purpose.

---

### Rule 3 — Master Data References

```text
Institution / HR / Student
           ¦
           ¦ Stable Identifier
           ?
           J
```

J shall not duplicate canonical identity lifecycle.

---

### Rule 4 — Internal J Transactions

No workflow shall require a distributed atomic transaction across multiple J bounded contexts.

For example:

```text
J05 GoodsAccepted
        ¦
        ¦ Event
        ?
J06 InventoryReceived
```

If J06 processing temporarily fails, the system shall retry or reconcile the event rather than roll back the already completed J05 acceptance decision.

---

## 44.5 Failure Handling

Multi-domain integration shall support:

- Idempotent Consumers
- Retry
- Dead-Letter / Failed-Message Handling
- Correlation Tracking
- Operational Reconciliation
- Compensating Commands
- Event Replay where architecturally supported
- Human Intervention for unresolved exceptions

Business state shall not be silently lost because an external consumer was temporarily unavailable.

---

# 45. J00 Completion Criteria

J00 is considered architecture-ready when:

- J01—J10 boundaries are accepted.
- Canonical ownership is agreed.
- Finance versus physical-resource ownership is explicit.
- Campus Operations versus inventory ownership is explicit.
- Operational versus financial Asset ownership is explicit.
- Supplier and Item Master ownership is singular.
- Multi-domain integration follows contract-based boundaries.
- Commands and events have defined ownership.
- Manual and configurable operating models are supported.
- Multi-campus requirements are supported.
- Audit and segregation-of-duties principles are established.
- Physical verification cannot silently alter resource truth.
- Disposal cannot bypass authorization and audit.
- Future automation does not become a baseline dependency.

---

# 46. Architecture Lock Statement

Upon formal approval, this document shall become the Level-2 architectural contract for:

> **Section J — Enterprise Resource Management**

All subsequent Section J workflow specifications:

```text
J01 — Procurement Planning & Sourcing
J02 — Vendor & Supplier Management
J03 — Purchase Requisition & Approval
J04 — Purchase Order Management
J05 — Goods Receipt, Inspection & Acceptance
J06 — Inventory & Warehouse Management
J07 — Stock Operations & Distribution
J08 — Asset Lifecycle Management
J09 — Physical Verification, Audit & Reconciliation
J10 — Disposal, Scrap & Write-Off
```

shall conform to the ownership boundaries, integration principles, architectural constraints, terminology, and governance decisions established by J00.

Any subsequent requirement that conflicts with J00 shall be treated as an **architecture change request** rather than being silently implemented inside an individual workflow document.

---

# 47. Final Architectural Position

Section J establishes Enterprise Resource Management as an independent enterprise capability within SutraERP.

Its canonical responsibility is:

```text
                     RESOURCE REQUIREMENT
                              ¦
                              ?
                         PROCUREMENT
                              ¦
                              ?
                           SUPPLIER
                              ¦
                              ?
                       PURCHASE ORDER
                              ¦
                              ?
                         GOODS RECEIPT
                              ¦
                              ?
                          INSPECTION
                              ¦
                              ?
                    INVENTORY / WAREHOUSE
                              ¦
                              ?
                   DISTRIBUTION / ASSIGNMENT
                         +---------+
                         ¦         ¦
                         ?         ?
                    CONSUMPTION   ASSET
                                   ¦
                                   ?
                              VERIFICATION
                                   ¦
                                   ?
                               RETIREMENT
                                   ¦
                                   ?
                                DISPOSAL
```

The financial consequences of this lifecycle are integrated with Section F.

The operational purposes of resources remain owned by the requesting domains.

The identities of institutions, campuses, employees, students, and other externally mastered entities remain owned by their respective canonical domains.

Section J therefore becomes the enterprise-wide **system of record for the physical institutional resource lifecycle**, without becoming the owner of financial accounting, academic operations, workforce management, campus service execution, or examination operations.

---

# 48. Enterprise Trust Certification Closure

Section J is certification-ready only when every Enterprise Resource Management workflow preserves the trust ownership model defined in this document and in J01 through J10.

## 48.1 Certification Ownership Matrix

| Trust Capability | Canonical Owner | Section J Certification Position |
|------------------|-----------------|----------------------------------|
| Authentication | Section A03 | Consumed by Section J; not re-owned. |
| Authorization and entitlement infrastructure | Section A04 | Consumed by Section J; Section J defines business permissions and segregation-of-duties constraints. |
| Workflow runtime | Section A05 | Consumed by Section J; Section J defines workflow states, approvals, and business transitions. |
| Event infrastructure | Section A06 | Consumed by Section J; Section J defines domain event semantics. |
| Audit and evidence infrastructure | Section A07 | Consumed by Section J; Section J emits auditable business facts and evidence references. |
| Procurement planning and sourcing | J01 | Section J-owned and complete. |
| Supplier master and supplier governance | J02 | Section J-owned and complete. |
| Purchase requisition and approval | J03 | Section J-owned and complete. |
| Purchase order lifecycle | J04 | Section J-owned and complete. |
| Goods receipt, inspection, and acceptance | J05 | Section J-owned and complete. |
| Inventory and warehouse stock ledger | J06 | Section J-owned and complete. |
| Stock movement, issue, return, transfer, and distribution | J07 | Section J-owned and complete. |
| Operational asset register and custody lifecycle | J08 | Section J-owned and complete. |
| Physical verification, audit, and reconciliation | J09 | Section J-owned and complete. |
| Disposal, scrap, and write-off business execution | J10 | Section J-owned and complete. |
| Financial accounting consequences | Section F | Integrated downstream; not re-owned by Section J. |

## 48.2 Certification Gate

Future Section J changes must not pass architecture review unless they satisfy all of the following conditions:

- Canonical ownership is explicitly identified.
- No Section J workflow duplicates Section A platform security, identity, authorization, workflow, event, or audit infrastructure.
- No Section J workflow duplicates Section F financial accounting ownership.
- Every approval-producing workflow defines traceable actor, role, tenant, institution, decision, timestamp, state transition, and evidence requirements.
- Every sensitive supplier, procurement, inventory, asset, verification, reconciliation, disposal, and write-off action enforces authorization and segregation-of-duties boundaries.
- Every workflow preserves tenant, institution, campus, warehouse, supplier, asset, inventory, and finance integration boundaries.
- Every workflow emits auditable domain events through the platform event and audit architecture.
- Every retention, archive, deletion, or recovery behavior protects audit evidence and financial traceability.

Failure of any certification gate condition is an architecture defect and requires correction before the affected Section J change may be considered production-ready.

---

**End of L2-J00 — Enterprise Resource Management Architecture & Governance**
