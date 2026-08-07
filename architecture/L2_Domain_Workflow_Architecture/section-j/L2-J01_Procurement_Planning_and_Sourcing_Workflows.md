# L2-J01 — Procurement Planning & Sourcing Workflows

**Repository:** SutraERP
**Section:** J — Enterprise Resource Management
**Domain:** J01 — Procurement Planning & Sourcing
**Document ID:** L2-J01
**Version:** 1.0
**Status:** Locked
**Classification:** Level 2 Domain Workflow Architecture
**Parent Architecture:** L2-J00 — Enterprise Resource Management Architecture & Governance
**Canonical Owner:** Procurement Planning & Sourcing
**Document Owner:** Enterprise Resource Management — Procurement Planning

---

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose |
| Scope | §4 Scope; §5 Out of Scope; §9.3 Planning Scope |
| Actors | §8 Actors & Roles |
| Entities | §32 Aggregate Catalogue; §32.2 Aggregate Summary; §32.3 ProcurementPlan Aggregate |
| Business Rules | §37 Business Rules & Invariants |
| Workflow | §10 Procurement Demand & Planning Workflow; §14 Quotation / RFQ Workflow; §15 RFP Workflow |
| State Machine | §9.5 Procurement Plan Lifecycle; §12.3 Sourcing Event Lifecycle; §22.3 Comparative Statement Lifecycle |
| Contracts | §27 Rate Contract & Framework Procurement; §27.2 Rate Contract; §27.3 Rate Contract Lifecycle |
| Events | §35 Domain Event Catalogue; §36 Consumed Event Catalogue; §12.2 Sourcing Event Creation |
| Dependencies | §50 Dependency Matrix |
| Locked Decisions | §48 Governance Decisions; §26.5 Committee Decision |

---

# Table of Contents

1. Purpose
2. Domain Mission
3. Architectural Position
4. Scope
5. Out of Scope
6. Institutional Operating Model
7. Architectural Principles
8. Actors & Roles
9. Procurement Planning Architecture
10. Procurement Demand & Planning Workflow
11. Procurement Method Determination
12. Sourcing Initiation
13. Direct & Simplified Procurement
14. Quotation / RFQ Workflow
15. RFP Workflow
16. Tender Procurement Workflow
17. Supplier Invitation & Participation
18. Supplier Response / Bid Submission
19. Bid Opening
20. Technical Evaluation
21. Commercial Evaluation
22. Combined Evaluation & Comparative Statement
23. Clarification & Negotiation
24. Supplier Selection & Award Recommendation
25. Split Award & Multi-Supplier Selection
26. Purchase / Tender Committee Workflow
27. Rate Contract & Framework Procurement
28. Single-Source / Proprietary Procurement
29. Emergency Procurement
30. External / Government Procurement Integration
31. Sourcing Amendment, Cancellation & Re-Sourcing
32. Aggregate Catalogue
33. Command Catalogue
34. Query & Read Model Catalogue
35. Domain Event Catalogue
36. Consumed Event Catalogue
37. Business Rules & Invariants
38. Authorization & Segregation of Duties
39. Configuration Architecture
40. Audit & Traceability
41. Exception & Reconciliation Architecture
42. Notifications
43. Documents & Evidence
44. Multi-Domain Integration
45. Reporting & KPIs
46. Future Extension Points
47. Architectural Constraints
48. Governance Decisions
49. Workflow Catalogue
50. Dependency Matrix
51. Completion Criteria

---

# 1. Purpose

J01 defines the canonical architecture for **Procurement Planning & Sourcing** within SutraERP.

The domain governs the institutional process through which an approved requirement is transformed into an evaluated and approved supplier-selection outcome.

Its responsibility begins before supplier commitment and ends before creation of the canonical Purchase Order.

The primary lifecycle is:

```text
Institutional Requirement
          ¦
          ?
Procurement Planning
          ¦
          ?
Approved Procurement Demand
          ¦
          ?
Procurement Method
          ¦
          ?
Sourcing
          ¦
          ?
Supplier Participation
          ¦
          ?
Evaluation
          ¦
          ?
Selection Recommendation
          ¦
          ?
Approval
          ¦
          ?
Approved Sourcing Outcome
          ¦
          ?
J04 Purchase Order Management
```

J01 shall support procurement practices ranging from simple purchasing used by smaller colleges to formal tender and committee-based procurement used by larger universities.

---

# 2. Domain Mission

The mission of J01 is:

> To provide a configurable, auditable, policy-driven procurement planning and supplier-sourcing capability capable of supporting both simple institutional purchasing and formal competitive procurement without assuming a single organisational or regulatory procurement model.

J01 shall support institutions including:

- Private Colleges
- Private Universities
- Government Colleges
- Public Universities
- Autonomous Institutions
- Multi-Campus Universities
- Institutions with Centralised Procurement
- Institutions with Decentralised Procurement
- Institutions using External Procurement Systems

The architecture shall therefore avoid hard-coding one institution's procurement procedure.

---

# 3. Architectural Position

J01 is the sourcing and procurement-planning bounded context within Section J.

```text
              Requesting Domains
                     ¦
                     ?
            J03 Purchase Requisition
                     ¦
                     ?
        +-------------------------+
        ¦          J01            ¦
        ¦ Procurement Planning &  ¦
        ¦        Sourcing         ¦
        +-------------------------+
                     ¦
          +----------+----------+
          ¦          ¦          ¦
          ?          ?          ?
         J02       Finance   Documents /
      Suppliers               Workflow
          ¦
          +---------------------
                     ¦
                     ?
            Approved Sourcing
                Outcome
                     ¦
                     ?
          J04 Purchase Order
```

J01 owns the sourcing decision process.

It does not own the supplier identity, procurement demand, financial budget, Purchase Order, Goods Receipt, or payment.

---

# 4. Scope

J01 owns the following capabilities.

## 4.1 Procurement Planning

- Procurement Plan Creation
- Procurement Plan Revision
- Planned Procurement Requirements
- Department / Campus Procurement Planning
- Requirement Consolidation
- Planned Procurement Scheduling
- Planned versus Actual Procurement Views

Formal procurement planning shall be configurable and may be disabled for institutions that do not require it.

---

## 4.2 Procurement Method Determination

J01 governs selection or recommendation of procurement methods such as:

- Direct Purchase
- Simplified Purchase
- Quotation-Based Purchase
- RFQ
- RFP
- Limited / Selective Tender
- Open Tender
- Rate Contract / Framework Procurement
- Single-Source / Proprietary Procurement
- Emergency Procurement
- External Procurement

Allowed methods and applicable thresholds shall be institution-configurable.

---

## 4.3 Sourcing

J01 owns:

- Sourcing Event
- Supplier Invitation
- RFQ
- RFP
- Tender Process
- Supplier Response Recording
- Bid / Quotation Opening
- Technical Evaluation
- Commercial Evaluation
- Comparative Evaluation
- Clarification
- Permitted Negotiation
- Supplier Recommendation
- Award Recommendation

---

## 4.4 Procurement Committee Coordination

Where institutional policy requires committee-based procurement, J01 shall support:

- Committee Reference
- Committee Membership Reference
- Meeting / Evaluation Participation
- Evaluation Decisions
- Recommendations
- Approval Evidence

J01 shall not become a general-purpose committee-management system.

---

## 4.5 External Procurement

J01 shall support procurement where some or all sourcing activities occur through:

- External Procurement Agencies
- Government Procurement Systems
- External E-Tender Platforms
- Institution-Approved External Processes

SutraERP may record:

- External Reference
- Procurement Method
- Status
- Documents
- Evaluation Evidence
- Outcome
- Selected Supplier

The system shall distinguish externally executed activity from activity actually executed within SutraERP.

---

# 5. Out of Scope

## 5.1 Supplier Master

J01 does not own supplier identity.

```text
J02
Supplier Master
     ¦
     ?
J01
Supplier Participation
```

Supplier registration, qualification, suspension, blacklisting, and canonical supplier status belong to J02.

---

## 5.2 Purchase Requisition

J01 does not own the Purchase Requisition.

J03 owns:

- Procurement Demand
- Purchase Requisition
- Requisition Approval
- Procurement Justification
- Budget Validation Coordination

J01 consumes approved procurement demand.

---

## 5.3 Purchase Order

J01 does not create the canonical Purchase Order.

```text
J01
Approved Supplier Selection
          ¦
          ?
J04
Purchase Order Management
```

J04 owns supplier commitment.

---

## 5.4 Budget and Accounting

J01 does not own:

- Institutional Budget
- Budget Balance
- Budget Commitment
- General Ledger
- Accounts Payable
- Payment
- Financial Posting

These remain Finance responsibilities.

---

## 5.5 Goods Receipt

Physical delivery, inspection, acceptance, and rejection belong to J05.

---

## 5.6 Inventory and Assets

J01 does not own:

- Item Master
- Stock
- Warehouse
- Asset Register

These belong to J06 and J08 according to J00 ownership boundaries.

---

## 5.7 Technical Domain Definitions

The requesting domain remains responsible for specialist technical requirements.

For example:

```text
Academic / Laboratory Domain
         ¦
         ¦ Technical Requirement
         ?
J01 Procurement Specification Reference
```

J01 may preserve the specification used for procurement but does not become the canonical owner of specialist academic, medical, engineering, security, or other domain semantics.

---

# 6. Institutional Operating Model

## 6.1 Configurable Procurement Complexity

SutraERP shall not force every institution to use the same procurement lifecycle.

Three broad operating profiles shall be supported.

### Simplified Procurement

Suitable for institutions or transactions requiring limited procurement formality.

```text
Approved Requirement
        ¦
        ?
Direct / Quotation Purchase
        ¦
        ?
Approval
        ¦
        ?
Supplier Selection
        ¦
        ?
Purchase Order
```

---

### Standard Competitive Procurement

```text
Approved Requirement
        ¦
        ?
RFQ / Sourcing
        ¦
        ?
Multiple Suppliers
        ¦
        ?
Quotation Comparison
        ¦
        ?
Recommendation
        ¦
        ?
Approval
        ¦
        ?
Purchase Order
```

---

### Formal Tender Procurement

```text
Approved Requirement
        ¦
        ?
Tender Preparation
        ¦
        ?
Publication / Invitation
        ¦
        ?
Bid Submission
        ¦
        ?
Bid Opening
        ¦
        ?
Technical Evaluation
        ¦
        ?
Commercial Evaluation
        ¦
        ?
Committee Recommendation
        ¦
        ?
Approval
        ¦
        ?
Award Outcome
        ¦
        ?
Purchase Order
```

These are configuration profiles rather than separate architectures.

---

## 6.2 Private and Public Institution Support

J01 shall support both private and public higher-education institutions.

However, J01 shall **not encode one statutory procurement regime as universal SutraERP behaviour**.

Instead, institutions shall configure applicable:

- Procurement Methods
- Monetary Thresholds
- Number of Quotations
- Approval Levels
- Tender Requirements
- Committee Requirements
- Evaluation Procedures
- Exception Authorities
- Required Documents

Where an institution is subject to external statutory or government procurement requirements, those requirements shall be represented through configuration and integration contracts.

---

## 6.3 Centralised Procurement

An institution may operate:

```text
Departments / Campuses
          ¦
          ?
Purchase Requisitions
          ¦
          ?
Central Procurement Office
          ¦
          ?
J01 Sourcing
```

J01 shall support consolidation across organisational units.

---

## 6.4 Decentralised Procurement

Institutions may permit authorised campuses or departments to perform procurement within delegated limits.

```text
Institution
    ¦
    +-- Campus A Procurement
    +-- Campus B Procurement
    +-- Central Procurement
```

Authorization and value limits shall determine permitted actions.

Canonical J01 records remain institutionally consistent.

---

## 6.5 Hybrid Procurement

Institutions may configure:

```text
Low-Value Procurement
        ?
Department / Campus

Medium-Value Procurement
        ?
Central Procurement

High-Value Procurement
        ?
Tender / Committee
```

Thresholds shall be configurable and effective-dated.

---

## 6.6 Procurement Performed Outside SutraERP

J01 shall support an `External Execution` model.

```text
Approved Procurement Requirement
            ¦
            ?
External Procurement Process
            ¦
      +-----------+
      ¦           ¦
 External       SutraERP
 Execution      Tracking
      ¦           ¦
      +-----------+
            ?
      Approved Outcome
            ¦
            ?
     J04 Purchase Order
```

SutraERP shall not fabricate internal bid-opening or evaluation records for activities that actually occurred externally.

Instead, it shall preserve external references and evidence.

---

# 7. Architectural Principles

## 7.1 Policy-Driven Procurement

Procurement behaviour shall be driven by institution-approved configuration.

Business rules shall not be scattered across UI screens or hard-coded individually for institutions.

---

## 7.2 Competitive Procurement Where Required

J01 shall support competitive procurement without assuming that competition is mandatory for every purchase.

Direct or single-source procurement may be permitted where configured.

Such exceptions shall remain auditable.

---

## 7.3 Lowest Price Is Not Automatically the Winner

Supplier selection may consider:

- Technical Compliance
- Price
- Delivery
- Warranty
- Support
- Supplier Capability
- Supplier Performance
- Other Approved Evaluation Criteria

J01 shall not hard-code:

```text
Lowest Price = Selected Supplier
```

Selection shall follow the configured evaluation method.

---

## 7.4 Simple Institutions Remain Simple

A college using basic quotation procurement shall not be forced to configure:

- Tender Committees
- Sealed Bids
- Multi-Stage Evaluation
- Complex Weighted Scoring
- Negotiation Rounds

These capabilities shall be activated only where required.

---

## 7.5 Formal Institutions Can Enforce Formality

A university may require:

```text
Threshold
    ?
Procurement Method
    ?
Minimum Competition
    ?
Committee
    ?
Evaluation
    ?
Approval Authority
```

J01 shall be capable of enforcing such configured policies.

---

## 7.6 Procurement Exceptions Are Explicit

An exception shall never be implemented by silently bypassing the normal workflow.

Examples include:

- Insufficient Quotations
- Single-Source Purchase
- Emergency Procurement
- Late Bid Acceptance
- Procurement Method Override
- Non-Lowest Supplier Selection

Each shall retain explicit authority and justification where applicable.

---

## 7.7 Historical Evidence Is Immutable

Once formally issued or completed, sourcing evidence shall not be destructively rewritten.

This includes:

- Supplier Invitations
- Supplier Responses
- Bid Versions
- Evaluations
- Comparative Statements
- Committee Recommendations
- Award Decisions

Corrections shall preserve prior history.

---

## 7.8 External Execution Must Be Truthful

Where procurement occurred outside SutraERP, the system shall record it as external procurement.

It shall not create artificial internal events implying SutraERP conducted activities that occurred elsewhere.

---

## 7.9 No Multi-Domain Ownership Leakage

J01 shall communicate through stable identifiers, commands, queries, and events.

It shall not directly modify J02, J03, J04, Finance, or requesting-domain aggregates.

---

# 8. Actors & Roles

J01 defines business roles without owning employee identity.

Typical roles include:

| Role | Responsibility |
|---|---|
| Procurement Planner | Prepares procurement plans |
| Procurement Officer | Manages sourcing activities |
| Procurement Manager | Supervises procurement |
| Requesting Unit Representative | Provides requirement/specification context |
| Technical Evaluator | Evaluates technical compliance |
| Commercial Evaluator | Performs commercial evaluation |
| Committee Member | Participates in configured committee decisions |
| Committee Chair / Convenor | Coordinates committee decision where required |
| Procurement Recommender | Records supplier recommendation |
| Procurement Approver | Approves sourcing outcome |
| External Procurement Coordinator | Records externally executed procurement |
| Auditor / Reviewer | Reviews procurement evidence |

One employee may perform multiple roles where institutional policy permits.

---

## 8.1 Segregation of Duties

Institutions shall be capable of enforcing:

```text
Requester
    ?
Evaluator
    ?
Final Approver
```

or more detailed separation:

```text
Procurement Officer
        ¦
        ?
Technical Evaluator
        ¦
        ?
Commercial Evaluator
        ¦
        ?
Recommendation Committee
        ¦
        ?
Approval Authority
```

Such separation shall be configurable rather than universally mandatory.

---

## 8.2 Supplier as External Participant

Suppliers may eventually interact through a Supplier Portal.

Baseline J01 shall not depend on supplier self-service.

Supplier responses may be recorded by authorised institutional procurement personnel when submissions occur through:

- Email
- Physical Documents
- External Portals
- Sealed Submission
- Other Approved Channels

The original submission channel and evidence shall remain traceable.

---

# 9. Procurement Planning Architecture

## 9.1 Purpose

Procurement Planning allows institutions to anticipate procurement requirements before individual sourcing activities begin.

Planning shall remain optional.

---

## 9.2 Planning Period

Procurement plans may be created for configurable periods such as:

- Financial Year
- Academic Year
- Semester
- Quarter
- Institution-Defined Period

J01 shall not assume that academic and financial planning periods are identical.

---

## 9.3 Planning Scope

A Procurement Plan may apply to:

```text
Institution
    ¦
    +-- Campus
    ¦
    +-- Department
    ¦
    +-- School / Faculty
    ¦
    +-- Other Organisational Unit
```

Organisational identities remain externally owned.

---

## 9.4 Planned Requirement

A planned procurement requirement may include:

- Resource / Service Description
- Existing Item Reference where applicable
- Category
- Estimated Quantity
- Unit of Measure
- Estimated Requirement Date
- Estimated Cost Reference
- Requesting Unit
- Purpose
- Priority
- Technical Specification Reference
- Proposed Procurement Method
- Budget Reference where available

Estimated values are planning information and shall not become authoritative financial expenditure.

---

## 9.5 Procurement Plan Lifecycle

```text
Draft
  ¦
  ?
Submitted
  ¦
  ?
Under Review
  ¦
  +--------? Returned for Revision
  ¦
  ?
Approved
  ¦
  ?
Active
  ¦
  +--------? Revised through controlled version
  ¦
  ?
Completed / Closed
```

Institutions not requiring approval may configure a simplified lifecycle.

---

## 9.6 Plan Revision

Approved plans shall not be silently overwritten.

Revision shall preserve:

- Previous Version
- New Version
- Change Reason
- Changed Requirements
- Actor
- Approval where required
- Effective Date

---

## 9.7 Unplanned Procurement

Procurement shall not be blocked merely because a requirement was absent from a procurement plan.

Where policy requires, an unplanned requirement may require:

- Additional Justification
- Additional Approval
- Budget Validation
- Procurement Authority Review

This allows institutions to maintain governance without making annual plans operationally inflexible.

---

## 9.8 Procurement Consolidation

Similar planned requirements may be consolidated.

Example:

```text
Computer Science Department -- 20 Laptops --+
                                            ¦
Administration --------------- 10 Laptops --+--? Consolidated Procurement
                                            ¦
Library ---------------------- 15 Laptops --+

                                            45 Laptops
```

The consolidated requirement shall retain traceability to its originating units.

---

## 9.9 Planning Does Not Replace J03

A Procurement Plan represents anticipated demand.

A J03 Purchase Requisition represents authorised procurement demand.

Therefore:

```text
Procurement Plan
       ¦
       ?
Planned Requirement
       ¦
       ?
J03 Purchase Requisition
       ¦
       ?
Approved Procurement Demand
       ¦
       ?
J01 Sourcing
```

Where institutional policy permits sourcing directly from an approved consolidated procurement authority, that path shall require explicit configuration and traceability.

J01 shall not silently bypass J03 approval semantics.
# 10. Procurement Demand & Planning Workflow

## 10.1 Purpose

This workflow governs how approved institutional procurement demand enters J01 and becomes eligible for sourcing.

J01 shall distinguish between:

- Planned Requirement
- Approved Purchase Requisition
- Consolidated Procurement Requirement
- Emergency Requirement
- External Procurement Requirement

The originating business requirement remains traceable throughout sourcing.

---

## 10.2 Standard Procurement Demand Entry

The standard workflow is:

```text
J03 Purchase Requisition Approved
              ¦
              ?
Procurement Demand Received by J01
              ¦
              ?
Validate Procurement Eligibility
              ¦
              ?
Determine Procurement Context
              ¦
       +------+---------+
       ¦      ¦         ¦
     Normal  Rate     Emergency /
             Contract  Exceptional
       ¦      ¦         ¦
       ?      ?         ?
 Procurement Method Determination
              ¦
              ?
       Sourcing Required?
           /       \
         Yes        No
          ¦          ¦
          ?          ?
     Sourcing       Direct /
      Event         Contract Route
```

---

## 10.3 Procurement Eligibility Validation

Before sourcing begins, J01 shall validate applicable:

- Purchase Requisition Status
- Procurement Authority
- Requesting Unit
- Procurement Category
- Required Date
- Quantity
- Technical Specification Availability
- Budget Validation Reference where required
- Procurement Plan Reference where applicable
- Existing Rate Contract Availability
- Emergency / Exception Classification

J01 shall not independently re-approve the Purchase Requisition.

---

## 10.4 Planned Requirement Linkage

Where a Purchase Requisition originated from a Procurement Plan:

```text
Procurement Plan
      ¦
      ?
Planned Requirement
      ¦
      ?
Purchase Requisition
      ¦
      ?
Sourcing Event
```

All references shall remain available for planned-versus-actual reporting.

---

## 10.5 Unplanned Demand

An approved Purchase Requisition may be classified as `Unplanned` where no applicable Procurement Plan entry exists.

Depending on institutional configuration, J01 may:

- Accept it normally.
- Require additional justification.
- Require Procurement Manager review.
- Require additional approval.
- Route it through an exception process.

Absence from a Procurement Plan shall not universally prohibit procurement.

---

## 10.6 Demand Consolidation

Multiple approved Purchase Requisitions may be sourced together where appropriate.

Example:

```text
PR-001 -- 20 Desktop Computers --+
                                 ¦
PR-014 -- 10 Desktop Computers --+--? Consolidated Sourcing Requirement
                                 ¦
PR-021 -- 15 Desktop Computers --+
```

Consolidation shall preserve:

- Original Requisition IDs
- Requesting Units
- Original Quantities
- Required Dates
- Budget References where applicable
- Allocation Requirements

Consolidation shall not erase originating demand.

---

## 10.7 Demand Splitting

A single Purchase Requisition may require multiple sourcing activities.

Example:

```text
Purchase Requisition

Laboratory Setup
       ¦
       +-- Equipment -----? Sourcing Event A
       +-- Furniture -----? Sourcing Event B
       +-- Installation --? Sourcing Event C
```

J01 shall preserve fulfilment allocation between requisition lines and sourcing events.

---

# 11. Procurement Method Determination

## 11.1 Purpose

Procurement Method Determination establishes the institution-approved process through which a procurement requirement shall be sourced.

The method shall be selected using configured procurement policy rather than hard-coded application behaviour.

---

## 11.2 Supported Procurement Methods

Baseline J01 shall support configuration of:

| Method | Typical Use |
|---|---|
| Direct Purchase | Permitted simple purchases |
| Simplified Purchase | Low-complexity institutional procurement |
| Quotation-Based Purchase | Comparison of supplier quotations |
| RFQ | Formal request for commercial quotations |
| RFP | Requirement where solution/proposal evaluation is important |
| Limited / Selective Tender | Procurement from selected eligible suppliers |
| Open Tender | Broad competitive procurement |
| Rate Contract / Framework | Procurement against existing approved arrangement |
| Single-Source Procurement | One justified supplier |
| Proprietary Procurement | Resource available from specific source/manufacturer |
| Emergency Procurement | Urgent exceptional requirement |
| External Procurement | Procurement executed through approved external mechanism |

Institutions may disable methods they do not use.

---

## 11.3 Procurement Method Rules

Method determination may consider:

- Estimated Procurement Value
- Item / Service Category
- Procurement Type
- Campus
- Requesting Unit
- Funding Source Reference
- Emergency Classification
- Existing Rate Contract
- Supplier Availability
- Institution Type
- Applicable Procurement Policy

---

## 11.4 Value-Based Procurement Policy

Institutions shall configure their own value bands.

Conceptually:

```text
Estimated Procurement Value
           ¦
           ?
   Procurement Policy
           ¦
   +-------+--------+
   ¦       ¦        ¦
Low      Medium    High
   ¦       ¦        ¦
   ?       ?        ?
Direct   RFQ /    Tender /
Purchase Quotes   Committee
```

No monetary threshold shall be hard-coded by J01.

This is particularly important because procurement rules may differ between institutions and may change over time.

---

## 11.5 Method Recommendation

J01 may determine a recommended procurement method from configured policy.

Example:

```text
Approved Requirement
        ¦
        ?
Evaluate Policy
        ¦
        ?
Recommended Method
        ¦
        ?
Procurement Officer Review
```

Where policy permits, the recommendation may be automatically accepted.

---

## 11.6 Procurement Method Override

An authorised user may override the recommended method only where configured.

An override shall record:

- Recommended Method
- Selected Method
- Override Reason
- Actor
- Timestamp
- Approval Authority where required

Example:

```text
Recommended:
Competitive RFQ
       ¦
       ?
Override Requested
       ¦
       ?
Emergency Justification
       ¦
       ?
Authorised?
   /       \
 No         Yes
 ¦           ¦
 ?           ?
Reject     Emergency Procurement
```

---

## 11.7 Effective-Dated Procurement Rules

Procurement policies shall support:

- Effective From
- Effective Until
- Policy Version
- Approval Reference
- Institution Scope

A sourcing event shall retain the procurement-policy version under which it was initiated.

Later policy changes shall not silently alter historical procurement decisions.

---

# 12. Sourcing Initiation

## 12.1 Purpose

A Sourcing Event is the J01 aggregate representing a controlled supplier-sourcing process.

A Sourcing Event may represent:

- Quotation Collection
- RFQ
- RFP
- Limited Tender
- Open Tender
- Other Configured Competitive Sourcing

---

## 12.2 Sourcing Event Creation

A Sourcing Event shall reference applicable:

- Sourcing Event ID
- Procurement Demand
- Purchase Requisition(s)
- Procurement Plan Reference
- Procurement Method
- Requesting Unit(s)
- Procurement Category
- Estimated Value Reference
- Required Completion Date
- Delivery Requirements
- Technical Specification References
- Applicable Procurement Policy
- Procurement Officer

---

## 12.3 Sourcing Event Lifecycle

A configurable baseline lifecycle is:

```text
Draft
  ¦
  ?
Preparation
  ¦
  ?
Approved for Publication / Invitation
  ¦
  ?
Open for Supplier Response
  ¦
  ?
Submission Closed
  ¦
  ?
Opening
  ¦
  ?
Evaluation
  ¦
  ?
Recommendation
  ¦
  ?
Approval
  ¦
  ?
Completed
```

Alternative terminal states include:

```text
Cancelled
No Response
No Suitable Bid
Re-Sourcing Required
```

Simplified procurement may omit unnecessary stages.

---

## 12.4 Sourcing Lines

A Sourcing Event may contain one or more lines.

Each line may contain:

- Item Reference where applicable
- Description
- Specification Reference
- Quantity
- Unit of Measure
- Required Delivery Date
- Delivery Location
- Technical Requirements
- Warranty Requirement
- Service Requirement
- Evaluation Classification

---

## 12.5 Existing versus New Items

Where procurement concerns an existing institutional Item:

```text
J06 Item Master
      ¦
      ?
J01 Sourcing Line
```

Where procurement concerns a resource not yet represented in the Item Master, J01 may source it using an approved specification.

Successful procurement may subsequently trigger controlled Item Master creation where inventory management is required.

J01 shall not create arbitrary J06 records directly.

---

## 12.6 Goods and Services

J01 shall support both:

```text
Goods Procurement
        ¦
        ?
Potential J05/J06/J08 Lifecycle
```

and:

```text
Service Procurement
        ¦
        ?
Service Confirmation /
Contractual Fulfilment
```

Service procurement shall not generate artificial inventory.

---

# 13. Direct & Simplified Procurement

## 13.1 Purpose

Not every institutional purchase requires a full competitive tender.

J01 shall provide a simplified procurement path for institutions and transaction classes where policy permits.

---

## 13.2 Simplified Workflow

```text
Approved Procurement Demand
            ¦
            ?
Simplified Procurement Eligible?
          /     \
        No       Yes
        ¦         ¦
        ?         ?
Standard       Supplier /
Sourcing       Quotation Selection
                  ¦
                  ?
             Justification
                  ¦
                  ?
               Approval
                  ¦
                  ?
          Approved Sourcing Outcome
                  ¦
                  ?
                 J04
```

---

## 13.3 Direct Purchase

Direct Purchase may be permitted based on configured:

- Value Threshold
- Category
- Procurement Type
- Urgency
- Supplier Arrangement
- Institutional Policy

Direct Purchase shall still retain:

- Procurement Requirement
- Supplier
- Selection Reason
- Approval where required
- Supporting Evidence
- Audit Trail

`Direct` shall not mean `uncontrolled`.

---

## 13.4 Simplified Quotation Comparison

Small procurement may use a lightweight quotation process.

Example:

```text
Requirement
    ¦
    ?
Quotation 1
Quotation 2
Quotation 3
    ¦
    ?
Simple Comparison
    ¦
    ?
Supplier Recommendation
    ¦
    ?
Approval
```

The number of required quotations shall be configurable.

---

# 14. Quotation / RFQ Workflow

## 14.1 Purpose

RFQ sourcing is used where the institution has sufficiently defined requirements and primarily requires suppliers to provide commercial and delivery quotations.

---

## 14.2 RFQ Preparation

An RFQ may contain:

- RFQ Reference
- Sourcing Event
- Requirement Description
- Item / Service Lines
- Quantities
- Technical Specification References
- Delivery Location
- Required Delivery Date
- Submission Deadline
- Commercial Requirements
- Warranty Requirements
- Required Supplier Documents
- Terms and Conditions References
- Contact / Clarification Process

---

## 14.3 Supplier Selection for Invitation

```text
J02 Eligible Suppliers
         ¦
         ?
Category / Capability Filter
         ¦
         ?
Procurement Officer Review
         ¦
         ?
Approved Invitation List
```

The invitation list shall preserve the supplier eligibility status applicable at the time of invitation.

---

## 14.4 Minimum Quotation Requirement

Institutions may configure minimum competition requirements.

Example:

```text
Required Quotations: 3
         ¦
         ?
Responses Received: 2
         ¦
      +-----+
      ¦     ¦
Continue?  Re-Solicit
      ¦
      ?
Exception Justification
      ¦
      ?
Approval where required
```

J01 shall not assume that insufficient supplier response must always terminate procurement.

The exception must remain visible and auditable.

---

## 14.5 RFQ Publication / Invitation

When an RFQ is issued, J01 shall record:

- RFQ Version
- Issue Date
- Invited Suppliers
- Submission Deadline
- Issuing Actor
- Delivery Method / Channel

Possible channels may include:

- Supplier Portal
- Email
- Physical Communication
- External Procurement System
- Other Institution-Approved Channel

---

## 14.6 RFQ Amendment

An issued RFQ may require amendment.

Amendment shall preserve:

```text
RFQ Version 1
      ¦
      ?
Amendment
      ¦
      ?
RFQ Version 2
```

The amendment shall record:

- Changed Information
- Reason
- Approval where required
- Amendment Date
- Revised Deadline where applicable

Previously issued versions shall remain historically accessible.

---

# 15. RFP Workflow

## 15.1 Purpose

RFP procurement supports requirements where suppliers are expected to propose a solution, approach, implementation, or service rather than merely quote against a fully specified item.

Examples may include institutionally procured:

- Professional Services
- Complex Technology Solutions
- Consultancy
- Managed Services
- Large Integrated Systems
- Other Solution-Based Procurement

---

## 15.2 RFP Structure

An RFP may include:

- Business Requirement
- Scope
- Deliverables
- Technical Requirements
- Implementation Expectations
- Service Requirements
- Commercial Submission Requirements
- Evaluation Criteria
- Submission Deadline
- Contractual References

---

## 15.3 RFP Evaluation

RFP evaluation may use:

```text
Technical Proposal
       ¦
       ?
Technical Evaluation
       ¦
       ?
Qualified Suppliers
       ¦
       ?
Commercial Evaluation
       ¦
       ?
Combined Evaluation
       ¦
       ?
Recommendation
```

Weighted evaluation shall be configurable.

---

# 16. Tender Procurement Workflow

## 16.1 Purpose

Tender procurement supports formal competitive procurement required by institutional policy.

J01 shall support tendering without assuming every institution follows the same statutory procedure.

---

## 16.2 Tender Types

Configurable tender methods may include:

- Open Tender
- Limited Tender
- Selective Tender
- Institution-Defined Tender Type

Single-source and emergency procurement remain separate exception methods.

---

## 16.3 Tender Workflow

```text
Approved Procurement Demand
          ¦
          ?
Tender Method Selected
          ¦
          ?
Tender Drafted
          ¦
          ?
Tender Approval
          ¦
          ?
Publication / Invitation
          ¦
          ?
Clarification Period
          ¦
          ?
Bid Submission
          ¦
          ?
Submission Deadline
          ¦
          ?
Bid Opening
          ¦
          ?
Technical Evaluation
          ¦
          ?
Commercial Evaluation
          ¦
          ?
Committee / Recommendation
          ¦
          ?
Approval
          ¦
          ?
Approved Sourcing Outcome
```

---

## 16.4 Tender Publication

Tender publication may occur through:

- SutraERP Supplier Portal in future
- Institutional Website
- Government Procurement Portal
- External E-Tender Platform
- Newspaper / Public Notice Reference
- Direct Supplier Invitation where method permits

J01 shall record publication evidence and references.

---

## 16.5 Tender Deadlines

Tender processes may define:

- Publication Date
- Clarification Deadline
- Submission Deadline
- Bid Opening Date
- Evaluation Period
- Expected Award Date

Deadline changes shall be versioned and communicated to applicable participants.

---

## 16.6 Tender Corrigendum

A corrigendum may modify:

- Specification
- Quantity
- Submission Conditions
- Eligibility Requirements
- Dates
- Evaluation Information
- Other Tender Terms

Every corrigendum shall preserve:

- Original Tender Version
- Changed Version
- Reason
- Authority
- Publication Date

---

## 16.7 Tender Cancellation

A tender may be cancelled due to:

- Requirement Withdrawn
- Material Specification Change
- Budget / Authority Change
- Insufficient Competition
- No Suitable Bid
- Administrative Decision
- Other Approved Reason

Cancellation shall not delete:

- Published Tender
- Supplier Responses
- Evaluation Work Already Performed
- Committee Evidence

The cancelled sourcing event remains part of procurement history.

---

# 17. Supplier Invitation & Participation

## 17.1 Supplier Eligibility

J01 shall consume supplier status from J02.

Depending on procurement method and configuration, suppliers may need to be:

- Registered
- Qualified
- Approved
- Active
- Eligible for Relevant Category

Suspended or blacklisted suppliers shall normally be prohibited from new sourcing participation.

---

## 17.2 Prospective Suppliers

Where policy permits, a prospective supplier may participate subject to subsequent or parallel J02 qualification.

J01 shall not create a second Supplier Master merely to accommodate such participation.

---

## 17.3 Supplier Invitation

Each invitation shall record:

- Supplier ID
- Sourcing Event ID
- Invitation Date
- Invitation Channel
- Invited By
- Applicable Sourcing Version
- Participation Status

---

## 17.4 Supplier Participation Status

Possible states include:

```text
Invited
   ¦
   +-- Accepted Invitation
   +-- Declined
   +-- No Response
   +-- Response Submitted
   +-- Withdrawn
```

Exact status terminology may be configured but the business history shall remain preserved.

---

# 18. Supplier Response / Bid Submission

## 18.1 Purpose

A Supplier Response represents the supplier's submitted commercial and/or technical response to a Sourcing Event.

J01 shall preserve each accepted submission as procurement evidence.

---

## 18.2 Response Information

Depending on procurement method, a response may contain:

- Supplier
- Response Version
- Submission Timestamp
- Submission Channel
- Quoted Lines
- Unit Prices
- Applicable Charges
- Discount References
- Delivery Timeline
- Warranty
- Technical Proposal
- Commercial Proposal
- Supporting Documents
- Validity Period
- Deviations / Exceptions
- Supplier Remarks

Financial accounting interpretation remains outside J01.

---

## 18.3 Line-Level Responses

Suppliers may quote:

```text
Sourcing Event
     ¦
     +-- Line 1 ? Supplier Price / Terms
     +-- Line 2 ? Supplier Price / Terms
     +-- Line 3 ? Supplier Price / Terms
```

Partial bidding may be enabled or disabled per sourcing event.

---

## 18.4 Revised Submission

Where policy permits, suppliers may revise their response before the submission deadline.

```text
Response V1
     ¦
     ?
Response V2
     ¦
     ?
Final Response
```

Earlier versions shall remain preserved.

---

## 18.5 Late Submission

Submissions received after the configured deadline shall be:

- Rejected automatically, or
- Recorded as late and held for authorised exception decision.

The behaviour shall be policy-driven.

Acceptance of a late response shall require explicit authority where enabled.

---

## 18.6 Response Withdrawal

A supplier may withdraw its response where procurement policy permits.

Withdrawal shall preserve:

- Original Response
- Withdrawal Time
- Withdrawal Reason where provided
- Actor / Source
- Status

Withdrawal shall not delete the historical bid.

---

## 18.7 Confidentiality

Supplier responses may be classified as restricted until the authorised opening stage.

Access shall be limited according to:

- Procurement Role
- Sourcing Event
- Evaluation Stage
- Committee Membership
- Institutional Policy

Commercial information shall not be exposed to competing suppliers through J01.

---

## 18.8 Sealed-Bid Behaviour

Where required, J01 shall support a logical sealed-bid model.

```text
Supplier Submission
        ¦
        ?
Received / Locked
        ¦
        ?
Submission Deadline
        ¦
        ?
Authorised Opening
        ¦
        ?
Evaluation
```

Baseline architecture shall also support physical sealed submissions by recording receipt evidence and opening information without pretending the bid contents were digitally sealed by SutraERP.
# 19. Bid Opening

## 19.1 Purpose

Bid Opening governs the controlled transition of supplier submissions from restricted receipt state into authorised evaluation.

Formal Bid Opening shall be configurable.

Simplified quotation procurement may proceed directly to evaluation where institutional policy does not require a separate opening stage.

---

## 19.2 Bid Opening Requirement

Whether formal Bid Opening is required may depend on:

- Procurement Method
- Procurement Value
- Tender Type
- Institutional Policy
- Funding / Scheme Requirements
- External Procurement Requirements

Example:

```text
Supplier Responses Closed
          ¦
          ?
Formal Opening Required?
       /         \
     No           Yes
     ¦             ¦
     ?             ?
Evaluation     Opening Scheduled
                    ¦
                    ?
             Authorised Opening
                    ¦
                    ?
                Evaluation
```

---

## 19.3 Opening Authority

Institutions may configure:

- Procurement Officer
- Procurement Manager
- Tender Committee
- Bid Opening Committee
- Named Authorised Officers
- Other Approved Authority

Where multiple participants are required, J01 shall support recording all participants.

---

## 19.4 Opening Controls

Formal opening may validate:

- Submission Deadline Passed
- Required Opening Time Reached
- Authorised Participants Present
- Supplier Submission Status
- Submission Integrity Evidence where digitally available
- Applicable Procurement Policy

The system shall not automatically declare a bid technically or commercially compliant merely because it was successfully opened.

---

## 19.5 Opening Record

A Bid Opening Record may contain:

- Sourcing Event ID
- Opening Date
- Opening Time
- Opening Location / Mode
- Participants
- Supplier Responses Opened
- Responses Rejected from Opening
- Late Submission Decisions
- Opening Remarks
- Supporting Evidence
- External Opening Reference where applicable

---

## 19.6 Physical Bid Opening

Where suppliers submit physical sealed bids, SutraERP may record:

```text
Physical Bid Received
        ¦
        ?
Receipt Evidence
        ¦
        ?
Secure Physical Custody
        ¦
        ?
Opening Meeting
        ¦
        ?
Opening Record
```

J01 shall not claim cryptographic or digital sealing for physical documents.

---

## 19.7 External E-Tender Opening

Where bids are opened through an external procurement platform:

```text
External Platform
      ¦
      ?
Bid Opening
      ¦
      ?
External Opening Evidence
      ¦
      ?
J01 External Opening Record
```

J01 records the result and reference without pretending the opening occurred within SutraERP.

---

# 20. Technical Evaluation

## 20.1 Purpose

Technical Evaluation determines whether supplier responses satisfy applicable technical, functional, eligibility, quality, or service requirements.

Technical Evaluation may occur before commercial evaluation.

---

## 20.2 Technical Evaluation Authority

Evaluation may be performed by:

- Procurement Personnel
- Requesting Department Representatives
- Subject-Matter Experts
- Technical Committee
- Tender Committee
- External Experts where authorised

Employee and organisational identities remain owned outside J01.

---

## 20.3 Evaluation Criteria

Criteria may include:

- Specification Compliance
- Mandatory Requirements
- Experience
- Certifications
- Technical Capability
- Proposed Methodology
- Implementation Approach
- Service Capability
- Warranty
- Support Capability
- Delivery Capability
- Institution-Defined Criteria

Criteria shall be configurable for the sourcing event.

---

## 20.4 Evaluation Methods

J01 shall support:

### Pass / Fail

```text
Criterion
   ¦
   +-- Compliant
   +-- Non-Compliant
```

### Scored Evaluation

```text
Criterion
     ¦
     ?
Score
     ¦
     ?
Weighted Score
```

### Combined Evaluation

A sourcing event may combine mandatory pass/fail criteria with scored criteria.

---

## 20.5 Technical Qualification

Supplier responses may result in:

- Technically Qualified
- Technically Disqualified
- Clarification Required
- Evaluation Pending

Disqualification shall require an explicit reason.

---

## 20.6 Commercial Information Restriction

Where policy requires technical evaluation before commercial opening:

```text
Technical Response
       ¦
       ?
Technical Evaluation
       ¦
       ?
Qualified?
   /       \
 No         Yes
 ¦           ¦
 ?           ?
Stop      Commercial
Commercial Evaluation
Evaluation
```

J01 shall support stage-based access restrictions so commercial information can remain unavailable until permitted.

---

## 20.7 Multiple Evaluators

Where multiple evaluators participate, J01 may support:

- Individual Evaluation
- Committee Evaluation
- Consolidated Evaluation
- Chair / Convenor Decision
- Consensus Recording

The configured evaluation method shall determine how individual evaluations become the final technical outcome.

---

## 20.8 Evaluation Revision

Before finalisation, authorised evaluators may revise their evaluation.

After finalisation, changes shall require a defined reopening/correction process.

Prior evaluation evidence shall remain retained.

---

# 21. Commercial Evaluation

## 21.1 Purpose

Commercial Evaluation compares the commercial aspects of technically eligible or otherwise valid supplier responses.

J01 performs procurement comparison.

It does not perform financial accounting.

---

## 21.2 Commercial Comparison

Comparison may include:

- Unit Price
- Total Quoted Price
- Discount
- Applicable Tax / Charge References
- Freight / Delivery Charges
- Installation Charges
- Warranty Terms
- Payment-Term References
- Delivery Schedule
- Commercial Validity
- Other Configured Commercial Factors

---

## 21.3 Normalised Comparison

Where supplier quotations use different commercial structures, J01 may provide a normalised comparative view according to configured procurement rules.

The original supplier quotation shall always remain preserved.

A normalised comparison shall not overwrite supplier-submitted commercial information.

---

## 21.4 Line-Level Comparison

Example:

```text
Item A

Supplier 1 -- Quoted Terms
Supplier 2 -- Quoted Terms
Supplier 3 -- Quoted Terms
      ¦
      ?
Comparative View
```

J01 shall support comparison at:

- Whole Response Level
- Sourcing Line Level

---

## 21.5 Lowest Evaluated Offer

J01 may identify the lowest evaluated commercial offer where relevant.

However:

> Lowest commercial price shall not automatically constitute supplier selection.

Selection remains subject to configured procurement policy and evaluation rules.

---

## 21.6 Commercial Deviations

Commercial deviations may include:

- Different Delivery Terms
- Different Warranty
- Different Payment Conditions
- Minimum Order Requirements
- Partial Quantity
- Alternative Commercial Conditions

Such deviations shall remain visible during evaluation.

---

# 22. Combined Evaluation & Comparative Statement

## 22.1 Purpose

J01 shall support creation of a structured comparative view combining applicable technical and commercial evaluation results.

---

## 22.2 Comparative Statement

A Comparative Statement may include:

- Supplier
- Technical Status
- Technical Score
- Commercial Value
- Delivery
- Warranty
- Supplier Performance Reference
- Deviations
- Overall Score where applicable
- Evaluator Remarks
- Ranking where configured

---

## 22.3 Comparative Statement Lifecycle

```text
Evaluation Data
      ¦
      ?
Draft Comparative Statement
      ¦
      ?
Review
      ¦
      ?
Finalised Comparative Statement
      ¦
      ?
Supplier Recommendation
```

Finalisation shall preserve the evaluation inputs used to produce the statement.

---

## 22.4 Weighted Evaluation

Where institutions use weighted evaluation:

```text
Technical Score
      ¦
      +----? Weight
      ¦
      ?
Weighted Technical Score

Commercial Score
      ¦
      +----? Weight
      ¦
      ?
Weighted Commercial Score

          ¦
          ?
Combined Score
```

Weighting methodology shall be configurable.

---

## 22.5 Supplier Performance

Where procurement policy permits, historical supplier performance from J02 may be considered.

J01 shall consume an approved supplier-performance representation rather than independently maintaining another supplier-performance system.

---

# 23. Clarification & Negotiation

## 23.1 Clarification

Procurement personnel or evaluators may request clarification from suppliers where permitted.

A clarification shall reference:

- Sourcing Event
- Supplier
- Supplier Response
- Question
- Requested By
- Request Date
- Response Deadline
- Supplier Response
- Supporting Documents
- Closure Status

---

## 23.2 Clarification Boundary

Clarification shall not silently become an unrestricted opportunity to replace a submitted bid.

Where a clarification materially changes the supplier offer, the applicable procurement policy shall determine whether:

- The change is permitted.
- A revised offer is required.
- All eligible suppliers must receive equivalent opportunity.
- Re-sourcing is required.

---

## 23.3 Negotiation

Negotiation shall be configurable.

Institutions whose procurement policy prohibits negotiation may disable it entirely.

Where permitted, negotiation may address:

- Price
- Delivery
- Warranty
- Support
- Commercial Terms
- Implementation Conditions
- Other Negotiable Elements

---

## 23.4 Negotiation Authority

Only configured roles may initiate or conduct negotiation.

J01 shall record:

- Participants
- Negotiation Date
- Negotiated Elements
- Supplier Response
- Revised Offer Reference
- Outcome
- Authority

---

## 23.5 Revised Offers

Negotiated offers shall not overwrite original supplier submissions.

```text
Original Bid
     ¦
     ?
Negotiation
     ¦
     ?
Revised Offer
     ¦
     ?
Final Evaluated Offer
```

All versions remain auditable.

---

## 23.6 Best and Final Offer

Where policy permits, J01 may support a Best and Final Offer round.

A BAFO round shall define:

- Eligible Suppliers
- Requested Revision Scope
- Submission Deadline
- Revised Responses
- Evaluation Method

This capability shall be optional.

---

# 24. Supplier Selection & Award Recommendation

## 24.1 Purpose

Supplier Selection determines the recommended supplier or suppliers resulting from the sourcing process.

J01 owns the recommendation and sourcing approval.

J04 owns the resulting Purchase Order.

---

## 24.2 Selection Inputs

Selection may consider:

- Technical Qualification
- Technical Score
- Commercial Evaluation
- Combined Score
- Delivery Capability
- Warranty
- Supplier Performance
- Procurement Policy
- Committee Recommendation
- Clarification / Negotiation Outcomes
- Approved Exceptions

---

## 24.3 Recommendation

A Supplier Recommendation shall record:

- Sourcing Event
- Recommended Supplier
- Recommended Lines
- Recommended Quantity
- Evaluated Commercial Reference
- Evaluation Basis
- Recommendation Reason
- Recommending Authority
- Supporting Comparative Statement

---

## 24.4 Non-Lowest Selection

Where the selected supplier is not the lowest commercial offer and policy requires explanation, J01 shall require explicit justification.

Possible legitimate factors may include:

- Technical Superiority
- Delivery Requirement
- Lifecycle / Support Consideration
- Warranty
- Evaluation Score
- Commercial Non-Compliance of Lower Offer
- Other Approved Evaluation Criteria

J01 shall not itself define which justification is legally sufficient for every institution.

---

## 24.5 Recommendation Approval

```text
Supplier Recommendation
          ¦
          ?
Approval Required?
      /        \
    No          Yes
    ¦            ¦
    ?            ?
Approved     Approval Queue
Outcome           ¦
             +---------+
             ¦         ¦
          Approved   Rejected /
             ¦       Returned
             ?         ¦
          Outcome      ?
                  Re-Evaluation
```

Approval authority may depend on:

- Procurement Value
- Procurement Method
- Category
- Campus
- Funding Context
- Exception Status
- Institutional Policy

---

## 24.6 Approved Sourcing Outcome

Once approved, J01 shall establish an `Approved Sourcing Outcome`.

The outcome shall contain sufficient information for J04 to create the applicable purchasing commitment.

J01 shall publish the outcome rather than directly inserting a Purchase Order into J04 storage.

---

# 25. Split Award & Multi-Supplier Selection

## 25.1 Purpose

A sourcing event may result in more than one supplier being selected.

---

## 25.2 Line-Level Award

Example:

```text
Sourcing Event

Line 1 -----? Supplier A
Line 2 -----? Supplier B
Line 3 -----? Supplier A
Line 4 -----? Supplier C
```

J01 shall support line-level supplier recommendations.

---

## 25.3 Quantity Split

Where policy permits, a quantity may be split.

```text
100 Units Required

Supplier A -- 60 Units
Supplier B -- 40 Units
```

The allocation and reason shall be recorded.

---

## 25.4 Multiple Purchase Orders

An approved split award may result in multiple J04 Purchase Orders.

```text
J01 Approved Sourcing Outcome
            ¦
       +----+----+
       ¦    ¦    ¦
       ?    ?    ?
      PO-A PO-B PO-C
           J04
```

J01 does not own those Purchase Orders.

---

# 26. Purchase / Tender Committee Workflow

## 26.1 Purpose

Indian higher-education institutions may use procurement, purchase, technical, or tender committees for specific procurement classes.

J01 shall support committee-driven procurement without making committees mandatory for all institutions.

---

## 26.2 Committee Requirement

Committee participation may be determined by:

- Procurement Method
- Procurement Value
- Category
- Institution Policy
- Exception Type
- Funding Requirement

---

## 26.3 Committee Composition

J01 may reference:

- Chair / Convenor
- Procurement Representative
- Finance Representative
- Technical Expert
- Requesting Department Representative
- External Expert
- Other Institution-Defined Member

Committee-member identity remains externally mastered.

---

## 26.4 Committee Actions

A procurement committee may participate in:

- Specification Review
- Bid Opening
- Technical Evaluation
- Commercial Evaluation
- Comparative Review
- Supplier Recommendation
- Procurement Exception Review

The exact authority shall be configured.

---

## 26.5 Committee Decision

A Committee Decision shall preserve:

- Committee Reference
- Meeting / Decision Date
- Participating Members
- Procurement Reference
- Recommendation
- Dissent / Remarks where recorded
- Supporting Documents
- Approval Evidence

---

## 26.6 Committee Quorum

Institutions may configure quorum requirements.

Where quorum is enabled:

```text
Committee Meeting
      ¦
      ?
Quorum Valid?
   /       \
 No         Yes
 ¦           ¦
 ?           ?
Cannot      Decision
Finalise    Permitted
```

Quorum rules shall not be hard-coded universally.

---

## 26.7 Committee Evidence

Minutes, signed recommendations, comparative statements, and related evidence may be retained through the enterprise document capability.

J01 shall retain stable references to that evidence.

---

# 27. Rate Contract & Framework Procurement

## 27.1 Purpose

J01 shall support sourcing arrangements where an institution has established approved suppliers, rates, or commercial terms for a defined validity period.

---

## 27.2 Rate Contract

A Rate Contract may define:

- Supplier
- Applicable Categories
- Item / Service References
- Agreed Rate References
- Valid From
- Valid Until
- Quantity / Value Ceiling where applicable
- Applicable Campuses
- Terms
- Approval Reference

---

## 27.3 Rate Contract Lifecycle

```text
Draft
  ¦
  ?
Review
  ¦
  ?
Approved
  ¦
  ?
Active
  ¦
  +--? Suspended
  ¦
  ?
Expired / Closed
```

---

## 27.4 Procurement Against Rate Contract

```text
Approved Requirement
       ¦
       ?
Applicable Active Rate Contract?
        /       \
      No         Yes
      ¦           ¦
      ?           ?
Normal          Validate
Sourcing        Eligibility
                  ¦
                  ?
            Approved Supplier
                  ¦
                  ?
                 J04
```

J04 remains responsible for the actual Purchase Order or call-off commitment.

---

## 27.5 Expiry

Expired rate contracts shall not normally be used for new purchasing commitments.

Authorised exceptions, where institutionally permitted, shall require explicit reason and authority.

---

## 27.6 Renewal

Renewal shall be treated as a controlled procurement decision.

J01 shall not silently extend an expired contract by modifying its end date.

---

# 28. Single-Source / Proprietary Procurement

## 28.1 Purpose

J01 shall support legitimate procurement cases where competitive sourcing is not used.

Examples may include:

- Proprietary Equipment
- Compatibility Requirement
- Sole Authorised Supplier
- Continuity Requirement
- Specialist Service
- Other Institution-Approved Reason

---

## 28.2 Single-Source Workflow

```text
Approved Requirement
       ¦
       ?
Single-Source Proposed
       ¦
       ?
Justification
       ¦
       ?
Supplier Eligibility
       ¦
       ?
Required Approval
       ¦
    +-----+
    ¦     ¦
 Approved Rejected
    ¦     ¦
    ?     ?
Source   Competitive /
Supplier Alternative Route
```

---

## 28.3 Mandatory Evidence

Where configured, single-source procurement shall require:

- Justification Category
- Detailed Reason
- Supplier
- Supporting Evidence
- Technical Confirmation
- Approval
- Applicable Policy Reference

---

## 28.4 Proprietary Certification

Where institutional policy requires proprietary or sole-source certification, J01 may record the certificate or supporting document reference.

The architecture shall not hard-code one certification format.

---

# 29. Emergency Procurement

## 29.1 Purpose

Emergency Procurement supports urgent institutional requirements where the normal procurement timeline would create unacceptable operational, safety, academic, legal, or institutional consequences.

Emergency procurement shall remain exceptional and auditable.

---

## 29.2 Emergency Workflow

```text
Urgent Approved Requirement
          ¦
          ?
Emergency Classification
          ¦
          ?
Emergency Justification
          ¦
          ?
Emergency Authority
          ¦
          ?
Abbreviated Procurement
          ¦
          ?
Supplier Selection
          ¦
          ?
Purchase Order / Permitted Commitment
          ¦
          ?
Post-Facto Review
       if required
```

---

## 29.3 Emergency Controls

J01 may require:

- Emergency Reason
- Urgency Category
- Requested By
- Emergency Authority
- Supplier Selection Reason
- Supporting Evidence
- Value
- Post-Facto Review
- Normalisation / Follow-Up Action

---

## 29.4 Emergency Procurement Reporting

Emergency procurement shall be separately identifiable for governance reporting.

Repeated emergency procurement for similar requirements may be surfaced for review.

The system shall not automatically infer misconduct from repeated emergency purchases.

---

# 30. External / Government Procurement Integration

## 30.1 Purpose

Institutions may conduct procurement using an external government, university, consortium, procurement-agency, or e-tender platform.

J01 shall support such procurement without requiring duplication of the complete external workflow.

---

## 30.2 External Procurement Record

J01 may record:

- External Procurement Reference
- External System / Agency
- Procurement Method
- Requirement Reference
- External Publication Reference
- Important Dates
- Participating Supplier References where available
- Evaluation Evidence
- Selected Supplier
- Outcome
- Supporting Documents
- External Status

---

## 30.3 External Execution Boundary

The system shall explicitly distinguish:

```text
Executed in SutraERP
```

from:

```text
Executed Externally
Recorded in SutraERP
```

This distinction is mandatory for audit integrity.

---

## 30.4 Partial External Execution

Hybrid processes shall be supported.

Example:

```text
J03 Approved Requirement
          ¦
          ?
J01 Procurement Initiation
          ¦
          ?
External E-Tender Process
          ¦
          ?
External Evaluation Outcome
          ¦
          ?
J01 Outcome Registration
          ¦
          ?
Institutional Approval
          ¦
          ?
J04 Purchase Order
```

---

## 30.5 External Integration

Future integrations may automate:

- Tender Publication
- Supplier Synchronisation
- Status Retrieval
- Bid / Outcome References
- Document Exchange

External integration shall not be a prerequisite for baseline J01 operation.

Manual recording of external references and evidence shall remain supported.

---

# 31. Sourcing Amendment, Cancellation & Re-Sourcing

## 31.1 Amendment

A sourcing event may be amended before completion where policy permits.

Material amendments shall preserve:

- Previous Version
- New Version
- Changed Fields
- Reason
- Approval
- Affected Suppliers
- Notification Status
- Deadline Changes

---

## 31.2 Cancellation

Cancellation shall require:

- Cancellation Reason
- Cancelling Authority
- Timestamp
- Applicable Approval
- Supplier Notification where applicable

Cancellation shall not delete procurement history.

---

## 31.3 No Response

Where no valid supplier response is received, J01 may conclude:

```text
No Response
```

Possible subsequent actions include:

- Re-Issue
- Invite Additional Suppliers
- Change Procurement Method through approval
- Cancel Requirement
- Escalate Exception

---

## 31.4 No Suitable Bid

Where responses exist but none are acceptable:

```text
Evaluation
    ¦
    ?
No Suitable Bid
    ¦
    +--? Re-Sourcing
    +--? Specification Review
    +--? Procurement Method Review
    +--? Requirement Cancellation
```

---

## 31.5 Re-Sourcing

A new sourcing event created after a failed or cancelled event shall retain a relationship to the previous event.

```text
Sourcing Event J01-SE-001
        ¦
        ?
No Suitable Bid
        ¦
        ?
Re-Sourcing
        ¦
        ?
Sourcing Event J01-SE-002
```

The second event shall not overwrite the first.

---

## 31.6 Closed Procurement Evidence

Completed, cancelled, failed, and superseded sourcing events shall remain historically queryable according to retention and access policy.

Destructive deletion shall not be used as a normal procurement correction mechanism.
# 32. Aggregate Catalogue

## 32.1 Purpose

This catalogue defines the principal transactional aggregates owned by J01.

An aggregate represents an authoritative transactional consistency boundary. Read models, reports, dashboards, external documents, and projections shall not be treated as aggregates merely because they expose J01 information.

---

## 32.2 Aggregate Summary

| Aggregate | Responsibility |
|---|---|
| ProcurementPlan | Institutional procurement planning and controlled revision |
| SourcingEvent | Canonical sourcing lifecycle |
| SupplierInvitation | Supplier participation invitation and status |
| SupplierResponse | Supplier quotation, proposal, bid, and response versions |
| BidOpeningRecord | Formal opening evidence where required |
| TechnicalEvaluation | Technical evaluation outcome |
| CommercialEvaluation | Commercial evaluation outcome |
| ComparativeEvaluation | Consolidated supplier comparison |
| SupplierClarification | Controlled supplier clarification |
| NegotiationRound | Permitted negotiation / revised-offer lifecycle |
| AwardRecommendation | Supplier-selection recommendation and approval |
| RateContract | Approved sourcing arrangement / framework |
| ProcurementException | Explicit sourcing exception governance |
| ExternalProcurementRecord | Procurement execution performed externally |

---

## 32.3 ProcurementPlan Aggregate

**Aggregate Root:** `ProcurementPlan`

Primary responsibilities:

- Planning Period
- Organisational Scope
- Planned Requirements
- Plan Status
- Plan Version
- Approval State
- Revision History

Conceptual structure:

```text
ProcurementPlan
¦
+-- PlanId
+-- InstitutionId
+-- PlanningPeriod
+-- Scope
+-- Status
+-- Version
¦
+-- PlannedRequirements[]
    +-- RequirementId
    +-- Description
    +-- ItemReference?
    +-- Category
    +-- EstimatedQuantity
    +-- EstimatedCostReference
    +-- ExpectedDate
    +-- RequestingUnit
    +-- Priority
    +-- SpecificationReference?
```

### Invariants

- Approved versions shall not be silently overwritten.
- Revision shall create controlled history.
- Estimated financial values shall not become Finance-owned balances.
- Closure shall not remove historical planned requirements.

---

## 32.4 SourcingEvent Aggregate

**Aggregate Root:** `SourcingEvent`

This is the principal J01 aggregate.

It owns:

- Procurement Method
- Sourcing Lifecycle
- Sourcing Lines
- Applicable Procurement Policy Reference
- Submission Dates
- Evaluation Configuration
- Sourcing Status
- Amendment History
- Cancellation / Failure Outcome

Conceptual structure:

```text
SourcingEvent
¦
+-- SourcingEventId
+-- InstitutionId
+-- ProcurementMethod
+-- Status
+-- PolicyReference
+-- ProcurementOfficer
+-- SubmissionDeadline
+-- RequisitionReferences[]
+-- PlanReferences[]
+-- SourcingLines[]
+-- Version
```

### Invariants

A Sourcing Event shall not:

- Exist without an authorised procurement basis.
- Lose traceability to originating demand.
- Be destructively rewritten after formal issue.
- Become a Purchase Order.
- Own canonical supplier identity.
- Own canonical budget state.

---

## 32.5 SupplierInvitation Aggregate

**Aggregate Root:** `SupplierInvitation`

Represents the controlled invitation of a supplier to participate in a Sourcing Event.

It may contain:

- Invitation ID
- Sourcing Event ID
- Supplier ID
- Sourcing Version
- Invitation Date
- Submission Deadline
- Invitation Channel
- Participation Status

SupplierInvitation shall reference J02 Supplier identity.

It shall not duplicate the Supplier Master.

---

## 32.6 SupplierResponse Aggregate

**Aggregate Root:** `SupplierResponse`

Represents the supplier's submitted response.

Possible response types include:

- Quotation
- RFQ Response
- Proposal
- Tender Bid
- Revised Offer
- Best and Final Offer

A SupplierResponse may contain:

```text
SupplierResponse
¦
+-- ResponseId
+-- SourcingEventId
+-- SupplierId
+-- Version
+-- SubmissionTimestamp
+-- SubmissionChannel
+-- ResponseLines[]
+-- TechnicalSubmissionReference?
+-- CommercialSubmissionReference?
+-- SupportingDocuments[]
+-- Status
```

### Invariants

- Original accepted submissions shall remain preserved.
- Revised responses shall create versions.
- Withdrawal shall not delete the response.
- Late submissions shall remain explicitly classified.
- Restricted response information shall follow stage-based authorization.

---

## 32.7 BidOpeningRecord Aggregate

Used only where formal Bid Opening applies.

It may contain:

- Opening ID
- Sourcing Event
- Opening Date / Time
- Opening Mode
- Participants
- Responses Opened
- Responses Not Opened
- Late Submission Decisions
- Remarks
- Evidence References

Bid Opening shall not itself determine technical qualification.

---

## 32.8 TechnicalEvaluation Aggregate

Represents the controlled technical evaluation of supplier responses.

It may contain:

- Evaluation ID
- Supplier Response
- Criteria
- Evaluators
- Scores
- Compliance Decisions
- Clarifications
- Technical Outcome
- Finalisation State

Possible outcomes include:

```text
Pending
Qualified
Disqualified
ClarificationRequired
```

---

## 32.9 CommercialEvaluation Aggregate

Represents commercial comparison for an eligible supplier response.

It may contain:

- Supplier Response Reference
- Commercial Evaluation Lines
- Evaluated Commercial Amount Reference
- Delivery Terms
- Warranty Terms
- Commercial Deviations
- Evaluation Status

The aggregate shall preserve the supplier's original submitted commercial information separately from any normalised comparison.

---

## 32.10 ComparativeEvaluation Aggregate

Represents the consolidated comparative evaluation used for recommendation.

It may combine:

- Technical Outcomes
- Technical Scores
- Commercial Outcomes
- Commercial Scores
- Delivery Factors
- Warranty Factors
- Supplier Performance References
- Overall Evaluation
- Ranking

It shall not automatically determine the selected supplier unless institutional policy explicitly defines deterministic selection.

---

## 32.11 SupplierClarification Aggregate

Represents a controlled clarification exchange.

It owns:

- Clarification Question
- Supplier
- Supplier Response
- Deadline
- Supporting Evidence
- Status

Clarification history shall remain auditable.

---

## 32.12 NegotiationRound Aggregate

Where negotiation is enabled, `NegotiationRound` owns:

- Negotiation Authority
- Participating Supplier(s)
- Negotiation Scope
- Negotiation Date
- Revised Offer References
- Outcome

Original offers remain immutable evidence.

---

## 32.13 AwardRecommendation Aggregate

Represents the proposed sourcing outcome before downstream Purchase Order creation.

It owns:

- Recommended Supplier(s)
- Recommended Lines
- Recommended Quantities
- Evaluation Basis
- Justification
- Approval Status
- Approval History

An approved AwardRecommendation may produce an `ApprovedSourcingOutcome`.

It shall not itself become a Purchase Order.

---

## 32.14 RateContract Aggregate

Where enabled, RateContract governs an approved sourcing arrangement.

It may contain:

- Supplier
- Validity
- Applicable Items / Categories
- Rate References
- Quantity / Value Ceilings
- Applicable Organisational Scope
- Status

J04 remains owner of individual purchasing commitments against the arrangement.

---

## 32.15 ProcurementException Aggregate

A ProcurementException may represent:

- Procurement Method Override
- Insufficient Quotations
- Late Bid Acceptance
- Single-Source Justification
- Emergency Procurement
- Competition Exception
- Other Configured Exception

It shall contain:

- Exception Type
- Business Context
- Reason
- Requested By
- Supporting Evidence
- Required Authority
- Decision
- Decision Actor
- Decision Timestamp

---

## 32.16 ExternalProcurementRecord Aggregate

Represents procurement execution occurring partly or completely outside SutraERP.

It may contain:

- External Reference
- External System / Agency
- Internal Requirement References
- Procurement Method
- External Status
- Important Dates
- Evidence
- Supplier Outcome
- Internal Approval State

It shall clearly identify which activities occurred externally.

---

# 33. Command Catalogue

## 33.1 Command Principles

Commands express business intent.

J01 commands shall:

- Identify the target aggregate.
- Carry authenticated actor context.
- Carry institution context.
- Enforce authorization.
- Enforce aggregate invariants.
- Support idempotency where integration requires it.
- Produce explicit success or rejection outcomes.

Multi-domain commands shall not be implemented as direct database writes.

---

## 33.2 Procurement Planning Commands

| Command | Purpose |
|---|---|
| CreateProcurementPlan | Create a new planning cycle |
| AddPlannedRequirement | Add anticipated procurement requirement |
| UpdatePlannedRequirement | Modify requirement while permitted |
| RemoveDraftPlannedRequirement | Remove requirement before protected state |
| SubmitProcurementPlan | Submit plan for review |
| ReturnProcurementPlanForRevision | Return submitted plan |
| ApproveProcurementPlan | Approve plan |
| ActivateProcurementPlan | Make approved plan operational |
| CreateProcurementPlanRevision | Create controlled revision |
| ApproveProcurementPlanRevision | Approve revised version |
| CloseProcurementPlan | Close completed planning cycle |

---

## 33.3 Sourcing Initiation Commands

| Command | Purpose |
|---|---|
| CreateSourcingEvent | Create sourcing from authorised demand |
| AddSourcingLine | Add procurement line |
| UpdateDraftSourcingLine | Modify draft line |
| LinkPurchaseRequisition | Link J03 demand |
| LinkPlannedRequirement | Link planning context |
| ConsolidateProcurementDemand | Consolidate multiple approved demands |
| AllocateDemandToSourcingEvent | Allocate requisition quantity to sourcing |
| DetermineProcurementMethod | Apply configured procurement policy |
| SelectProcurementMethod | Confirm method |
| RequestProcurementMethodOverride | Request exception |
| ApproveProcurementMethodOverride | Approve method exception |
| RejectProcurementMethodOverride | Reject method exception |

---

## 33.4 Sourcing Publication Commands

| Command | Purpose |
|---|---|
| PrepareRFQ | Prepare RFQ sourcing |
| PrepareRFP | Prepare RFP sourcing |
| PrepareTender | Prepare tender sourcing |
| SubmitSourcingForPublicationApproval | Request issue/publication approval |
| ApproveSourcingPublication | Approve sourcing issue |
| PublishSourcingEvent | Mark authorised publication |
| IssueSupplierInvitations | Issue selected supplier invitations |
| AmendSourcingEvent | Create controlled amendment |
| PublishSourcingAmendment | Issue amendment/corrigendum |
| ExtendSubmissionDeadline | Change deadline through controlled action |

---

## 33.5 Supplier Participation Commands

| Command | Purpose |
|---|---|
| InviteSupplier | Invite J02 supplier |
| RecordSupplierInvitationAcceptance | Record participation |
| RecordSupplierDecline | Record supplier decline |
| RecordSupplierNoResponse | Record no participation |
| RecordSupplierWithdrawal | Record withdrawal |

---

## 33.6 Supplier Response Commands

| Command | Purpose |
|---|---|
| RecordSupplierResponse | Record quotation/bid/proposal |
| SubmitSupplierResponse | Finalise digitally supported submission |
| RecordPhysicalBidReceipt | Record physical submission |
| RecordExternalBidReference | Reference externally submitted bid |
| RecordRevisedSupplierResponse | Preserve revised response |
| WithdrawSupplierResponse | Record withdrawal |
| ClassifyLateSubmission | Mark submission as late |
| RequestLateSubmissionAcceptance | Request exception |
| ApproveLateSubmissionAcceptance | Accept authorised late response |
| RejectLateSubmission | Reject late response |
| CloseSupplierSubmissions | Close submission period |

---

## 33.7 Bid Opening Commands

| Command | Purpose |
|---|---|
| ScheduleBidOpening | Establish opening context |
| RecordBidOpeningParticipants | Record authorised participants |
| OpenSupplierResponse | Record authorised opening |
| RecordPhysicalBidOpening | Record physical bid opening |
| RecordExternalBidOpening | Record external opening evidence |
| FinaliseBidOpening | Complete opening record |

---

## 33.8 Technical Evaluation Commands

| Command | Purpose |
|---|---|
| CreateTechnicalEvaluation | Begin technical evaluation |
| AssignTechnicalEvaluator | Assign authorised evaluator |
| RecordTechnicalCriterionDecision | Record pass/fail criterion |
| RecordTechnicalScore | Record scored criterion |
| RequestTechnicalClarification | Initiate clarification |
| QualifySupplierTechnically | Record qualification |
| DisqualifySupplierTechnically | Record disqualification |
| FinaliseTechnicalEvaluation | Finalise technical result |
| ReopenTechnicalEvaluation | Controlled reopening |

---

## 33.9 Commercial Evaluation Commands

| Command | Purpose |
|---|---|
| CreateCommercialEvaluation | Begin commercial evaluation |
| RecordCommercialComparison | Record evaluated commercial information |
| RecordCommercialDeviation | Capture deviation |
| RecordDeliveryEvaluation | Evaluate delivery terms |
| RecordWarrantyEvaluation | Evaluate warranty terms |
| FinaliseCommercialEvaluation | Finalise commercial result |
| ReopenCommercialEvaluation | Controlled reopening |

---

## 33.10 Comparative Evaluation Commands

| Command | Purpose |
|---|---|
| CreateComparativeStatement | Create consolidated comparison |
| ApplyEvaluationWeights | Apply configured weighting |
| RecordOverallEvaluation | Record combined evaluation |
| RankSupplierResponses | Produce configured ranking |
| FinaliseComparativeStatement | Protect final comparison |

Ranking shall not automatically equal award unless policy explicitly defines that behaviour.

---

## 33.11 Clarification & Negotiation Commands

| Command | Purpose |
|---|---|
| RequestSupplierClarification | Send controlled clarification |
| RecordSupplierClarificationResponse | Record supplier response |
| CloseSupplierClarification | Close clarification |
| InitiateNegotiationRound | Begin authorised negotiation |
| RecordNegotiationOutcome | Record negotiation result |
| RequestRevisedOffer | Request permitted revision |
| RecordNegotiatedOffer | Record revised offer |
| InitiateBestAndFinalOfferRound | Begin BAFO where permitted |
| CloseNegotiationRound | Finalise negotiation |

---

## 33.12 Award Commands

| Command | Purpose |
|---|---|
| CreateAwardRecommendation | Create supplier recommendation |
| RecommendSupplier | Recommend supplier |
| RecommendSupplierForLine | Recommend line-level supplier |
| AllocateAwardQuantity | Split quantity where permitted |
| RecordNonLowestSelectionJustification | Record required justification |
| SubmitAwardRecommendation | Submit for approval |
| ApproveAwardRecommendation | Approve recommendation |
| RejectAwardRecommendation | Reject recommendation |
| ReturnAwardRecommendationForReevaluation | Return to evaluation |
| FinaliseApprovedSourcingOutcome | Establish approved sourcing result |

---

## 33.13 Committee Commands

| Command | Purpose |
|---|---|
| AssociateProcurementCommittee | Associate configured committee |
| RecordCommitteeParticipants | Record participating members |
| ValidateCommitteeQuorum | Validate configured quorum |
| RecordCommitteeRecommendation | Record committee outcome |
| RecordCommitteeDissent | Preserve dissent where recorded |
| FinaliseCommitteeDecision | Complete committee decision |

---

## 33.14 Rate Contract Commands

| Command | Purpose |
|---|---|
| CreateRateContract | Create arrangement |
| AddRateContractLine | Add applicable resource/service |
| SubmitRateContractForApproval | Submit arrangement |
| ApproveRateContract | Approve arrangement |
| ActivateRateContract | Activate approved arrangement |
| SuspendRateContract | Suspend new usage |
| CloseRateContract | Close arrangement |
| ExpireRateContract | Mark expiry |
| CreateRateContractRenewal | Begin controlled renewal |

---

## 33.15 Emergency / Exception Commands

| Command | Purpose |
|---|---|
| DeclareEmergencyProcurement | Classify procurement as emergency |
| RecordEmergencyJustification | Record reason |
| ApproveEmergencyProcurement | Authorise abbreviated route |
| RecordEmergencySupplierSelection | Record selection |
| SubmitEmergencyProcurementForPostReview | Initiate configured post-facto review |
| CompleteEmergencyProcurementReview | Complete review |
| DeclareSingleSourceProcurement | Initiate single-source route |
| RecordSingleSourceJustification | Record basis |
| ApproveSingleSourceProcurement | Authorise route |

---

## 33.16 External Procurement Commands

| Command | Purpose |
|---|---|
| CreateExternalProcurementRecord | Register external execution |
| RecordExternalProcurementReference | Add external identifier |
| RecordExternalProcurementStatus | Update external status |
| AttachExternalProcurementEvidence | Associate evidence |
| RecordExternalEvaluationOutcome | Record externally produced evaluation |
| RecordExternalSupplierSelection | Record external selection |
| SubmitExternalOutcomeForInternalApproval | Request institutional approval |
| ApproveExternalProcurementOutcome | Approve recognised outcome |

---

## 33.17 Cancellation & Re-Sourcing Commands

| Command | Purpose |
|---|---|
| CancelSourcingEvent | Cancel event |
| DeclareNoResponseOutcome | Close with no response |
| DeclareNoSuitableBidOutcome | Close with no acceptable bid |
| RequestReSourcing | Request new sourcing cycle |
| CreateReSourcingEvent | Create linked successor event |
| CloseSourcingEvent | Close completed sourcing |

---

# 34. Query & Read Model Catalogue

## 34.1 Principle

Queries shall not mutate transactional state.

J01 may expose purpose-built read models for requesters, procurement teams, committees, approvers, auditors, and enterprise dashboards.

---

## 34.2 Procurement Planning Queries

Representative queries include:

- GetProcurementPlan
- GetProcurementPlanVersionHistory
- GetPlannedRequirements
- GetProcurementPlansByCampus
- GetProcurementPlansByDepartment
- GetUpcomingPlannedRequirements
- GetUnplannedProcurement
- GetPlannedVsActualProcurement

---

## 34.3 Sourcing Queries

Representative queries include:

- GetSourcingEvent
- GetSourcingEventTimeline
- GetSourcingEventLines
- GetSourcingEventVersionHistory
- GetSourcingEventsByStatus
- GetSourcingEventsByProcurementMethod
- GetSourcingEventsByCampus
- GetSourcingEventsByDepartment
- GetSourcingEventsByRequisition
- GetSourcingEventsAwaitingPublication
- GetSourcingEventsClosingSoon
- GetDelayedSourcingEvents

---

## 34.4 Supplier Participation Queries

Representative queries include:

- GetInvitedSuppliers
- GetSupplierParticipationStatus
- GetResponsesReceived
- GetResponsesPending
- GetDeclinedSuppliers
- GetSupplierResponseVersions
- GetLateSubmissions

Supplier-sensitive information shall respect authorization.

---

## 34.5 Evaluation Queries

Representative queries include:

- GetTechnicalEvaluation
- GetCommercialEvaluation
- GetComparativeStatement
- GetEvaluationProgress
- GetPendingTechnicalEvaluations
- GetPendingCommercialEvaluations
- GetTechnicallyQualifiedSuppliers
- GetDisqualifiedSuppliers
- GetEvaluationHistory

---

## 34.6 Award Queries

Representative queries include:

- GetAwardRecommendation
- GetPendingAwardApprovals
- GetApprovedSourcingOutcome
- GetSupplierAwardHistory
- GetSplitAwardAllocation
- GetNonLowestSelectionCases

---

## 34.7 Committee Queries

Representative queries include:

- GetProcurementCommitteeContext
- GetCommitteeParticipants
- GetCommitteeDecision
- GetCommitteeDecisionHistory
- GetCommitteeActionsPending

---

## 34.8 Exception Queries

Representative queries include:

- GetProcurementExceptions
- GetEmergencyProcurementCases
- GetSingleSourceCases
- GetMethodOverrides
- GetInsufficientQuotationExceptions
- GetLateBidExceptions
- GetExceptionsPendingApproval

---

## 34.9 External Procurement Queries

Representative queries include:

- GetExternalProcurementRecord
- GetExternalProcurementByReference
- GetExternallyExecutedSourcing
- GetExternalProcurementPendingInternalApproval
- GetExternalProcurementEvidence

---

## 34.10 Requester Procurement Tracking Read Model

Requesting departments may receive a simplified view:

```text
Requirement Approved
       ¦
       ?
Sourcing Preparation
       ¦
       ?
Supplier Responses
       ¦
       ?
Evaluation
       ¦
       ?
Approval
       ¦
       ?
Purchase Order Initiated
```

The requester shall not automatically receive access to:

- Confidential Supplier Bids
- Restricted Commercial Information
- Evaluator Notes
- Committee-Restricted Information

Visibility shall be role and policy driven.

---

## 34.11 Procurement Workbench

A procurement officer read model may include:

```text
Procurement Workbench

Planning
+-- Plans Pending Review
+-- Upcoming Requirements

Sourcing
+-- Draft Events
+-- Awaiting Publication
+-- Open Events
+-- Closing Soon
+-- Delayed Events

Evaluation
+-- Awaiting Opening
+-- Technical Evaluation
+-- Commercial Evaluation
+-- Recommendation Pending

Exceptions
+-- Insufficient Responses
+-- Method Overrides
+-- Emergency Cases
```

The workbench is a projection, not a new aggregate.

---

# 35. Domain Event Catalogue

## 35.1 Event Principles

J01 shall publish business events representing completed domain facts.

Events shall:

- Use stable event contracts.
- Carry institution context.
- Carry aggregate identity.
- Carry correlation / causation metadata.
- Be idempotently consumable.
- Avoid exposing unnecessary sensitive bid content.
- Avoid requiring distributed transactions.

---

## 35.2 Procurement Planning Events

Representative events include:

```text
ProcurementPlanCreated
PlannedRequirementAdded
ProcurementPlanSubmitted
ProcurementPlanReturnedForRevision
ProcurementPlanApproved
ProcurementPlanActivated
ProcurementPlanRevisionCreated
ProcurementPlanRevisionApproved
ProcurementPlanClosed
```

---

## 35.3 Sourcing Events

```text
SourcingEventCreated
ProcurementDemandConsolidated
ProcurementMethodDetermined
ProcurementMethodOverrideRequested
ProcurementMethodOverrideApproved
ProcurementMethodOverrideRejected
SourcingEventApprovedForPublication
SourcingEventPublished
SourcingEventAmended
SubmissionDeadlineExtended
SupplierSubmissionsClosed
```

---

## 35.4 Supplier Participation Events

```text
SupplierInvited
SupplierInvitationAccepted
SupplierInvitationDeclined
SupplierResponseRecorded
SupplierResponseSubmitted
SupplierResponseRevised
SupplierResponseWithdrawn
LateSupplierResponseDetected
LateSupplierResponseAccepted
LateSupplierResponseRejected
```

---

## 35.5 Bid Opening Events

```text
BidOpeningScheduled
SupplierResponseOpened
PhysicalBidOpeningRecorded
ExternalBidOpeningRecorded
BidOpeningFinalised
```

---

## 35.6 Evaluation Events

```text
TechnicalEvaluationStarted
TechnicalClarificationRequested
SupplierTechnicallyQualified
SupplierTechnicallyDisqualified
TechnicalEvaluationFinalised

CommercialEvaluationStarted
CommercialDeviationRecorded
CommercialEvaluationFinalised

ComparativeStatementCreated
ComparativeStatementFinalised
```

---

## 35.7 Clarification & Negotiation Events

```text
SupplierClarificationRequested
SupplierClarificationReceived
SupplierClarificationClosed

NegotiationRoundStarted
RevisedOfferRequested
NegotiatedOfferRecorded
BestAndFinalOfferRoundStarted
NegotiationRoundClosed
```

---

## 35.8 Award Events

```text
AwardRecommendationCreated
SupplierRecommended
SplitAwardRecommended
AwardRecommendationSubmitted
AwardRecommendationApproved
AwardRecommendationRejected
AwardRecommendationReturnedForReevaluation
ApprovedSourcingOutcomeFinalised
```

`ApprovedSourcingOutcomeFinalised` is the principal event available for J04 Purchase Order processing.

---

## 35.9 Committee Events

```text
ProcurementCommitteeAssociated
CommitteeQuorumValidated
ProcurementCommitteeRecommendationRecorded
CommitteeDecisionFinalised
```

---

## 35.10 Rate Contract Events

```text
RateContractCreated
RateContractApproved
RateContractActivated
RateContractSuspended
RateContractExpired
RateContractClosed
RateContractRenewalCreated
```

---

## 35.11 Exception Events

```text
EmergencyProcurementDeclared
EmergencyProcurementApproved
EmergencyProcurementReviewCompleted

SingleSourceProcurementDeclared
SingleSourceProcurementApproved

ProcurementExceptionRaised
ProcurementExceptionApproved
ProcurementExceptionRejected
```

---

## 35.12 External Procurement Events

```text
ExternalProcurementRegistered
ExternalProcurementStatusRecorded
ExternalEvaluationOutcomeRecorded
ExternalSupplierSelectionRecorded
ExternalProcurementOutcomeApproved
```

---

## 35.13 Closure Events

```text
SourcingEventCancelled
NoResponseOutcomeDeclared
NoSuitableBidOutcomeDeclared
ReSourcingRequested
ReSourcingEventCreated
SourcingEventClosed
```

---

# 36. Consumed Event Catalogue

## 36.1 Purpose

J01 consumes facts from other canonical domains where asynchronous integration is appropriate.

Consumption of an event shall not transfer ownership of the originating aggregate to J01.

---

## 36.2 J03 — Purchase Requisition Events

J01 may consume applicable events such as:

```text
PurchaseRequisitionApproved
PurchaseRequisitionAmended
PurchaseRequisitionCancelled
```

The exact contract names shall align with J03 when that domain architecture is authored.

J01 shall not invent a second Purchase Requisition lifecycle.

---

## 36.3 J02 — Supplier Events

J01 may consume supplier facts such as:

```text
SupplierApproved
SupplierQualificationChanged
SupplierSuspended
SupplierReactivated
SupplierBlacklisted
SupplierPerformanceUpdated
```

Exact contracts shall be aligned with J02.

---

## 36.4 Finance Events

Where useful, J01 may consume financial-status facts such as:

```text
BudgetValidationCompleted
BudgetAuthorityChanged
ProcurementFinancialHoldPlaced
ProcurementFinancialHoldReleased
```

Exact event contracts remain Finance-owned.

J01 shall not derive an independent authoritative budget balance from these events.

---

## 36.5 J06 — Item Events

Where existing Item Master references are used, J01 may consume applicable facts such as:

```text
ItemCreated
ItemClassificationChanged
ItemDeactivated
```

J06 remains canonical owner of the Item Master.

---

# 37. Business Rules & Invariants

## 37.1 Authorised Demand

A standard sourcing process shall originate from authorised procurement demand.

Draft or rejected requisitions shall not become normal sourcing authority.

---

## 37.2 Procurement Method

Every sourcing event shall have an identifiable procurement method.

Any policy-required method override shall remain explicit and auditable.

---

## 37.3 Supplier Identity

Every supplier participating in a sourcing event shall resolve to an appropriate J02 supplier/prospective-supplier identity according to configured policy.

J01 shall not create a competing Supplier Master.

---

## 37.4 Competition Requirements

Where procurement policy requires a minimum level of competition, J01 shall validate that requirement.

Failure to achieve minimum competition shall result in:

- Re-Solicitation,
- Approved Exception,
- Alternative Procurement Route, or
- Cancellation.

The requirement shall not be silently ignored.

---

## 37.5 Submission Deadline

A supplier response shall retain its actual receipt/submission timestamp.

Changing a sourcing deadline after a response arrives shall not rewrite the historical submission timestamp.

---

## 37.6 Confidentiality

Restricted supplier responses shall not become visible merely because they exist in the system.

Access shall follow procurement stage and authorization.

---

## 37.7 Evaluation Independence

Technical and commercial evaluation shall remain separable where procurement policy requires sequential evaluation.

---

## 37.8 Evaluation Evidence

A supplier shall not be disqualified without a recorded basis.

Final evaluation results shall retain evidence of the criteria and decisions used.

---

## 37.9 Award Recommendation

An Award Recommendation shall not exist without an applicable evaluation or explicitly authorised simplified/single-source procurement basis.

---

## 37.10 Lowest Price

The architecture shall never enforce the universal invariant:

```text
Lowest Quotation = Award Winner
```

Selection shall follow the applicable evaluation policy.

---

## 37.11 Split Award

Split awards shall only be permitted where the sourcing method and institutional policy allow them.

The allocation shall be explicit.

---

## 37.12 Historical Responses

Supplier responses shall not be destructively edited after protected submission.

Revision shall create additional history.

---

## 37.13 External Procurement

Externally executed procurement shall remain explicitly identified as externally executed.

SutraERP shall not fabricate internal workflow evidence.

---

## 37.14 Rate Contract Validity

New purchasing activity shall not normally use an expired or suspended Rate Contract.

Any permitted exception shall be explicit.

---

## 37.15 Emergency Procurement

Emergency classification shall not erase procurement accountability.

At minimum, the emergency basis and authorised decision shall remain traceable.

---

## 37.16 Cancellation

Cancellation shall not destroy supplier submissions, evaluations, or procurement evidence already created.

---

## 37.17 Re-Sourcing

A re-sourcing event shall retain traceability to the prior sourcing event.

---

## 37.18 Financial Boundary

J01 shall not:

- Reserve authoritative budget balances.
- Post accounting entries.
- Create Accounts Payable liabilities.
- Execute supplier payment.

---

## 37.19 Purchase Order Boundary

An approved sourcing outcome authorises downstream purchasing activity.

It does not itself constitute the canonical Purchase Order.

---

## 37.20 Multi-Domain Consistency

J01 shall not require synchronous distributed transactions with J02, J03, J04, J06, Finance, or other domains.

Integration failures shall be handled through retry, idempotency, reconciliation, or explicit exception handling.

---

# 38. Authorization & Segregation of Duties

## 38.1 Authorization Model

Authorization may consider:

- Institution
- Campus
- Organisational Unit
- Procurement Category
- Procurement Method
- Procurement Value Band
- Sourcing Event
- Assigned Role
- Committee Membership
- Delegated Authority
- Procurement Stage

---

## 38.2 Representative Permissions

J01 may expose permissions such as:

```text
procurement.plan.create
procurement.plan.review
procurement.plan.approve

procurement.sourcing.create
procurement.sourcing.publish
procurement.sourcing.amend
procurement.sourcing.cancel

procurement.bid.record
procurement.bid.open

procurement.technical.evaluate
procurement.commercial.evaluate

procurement.recommend
procurement.award.approve

procurement.exception.request
procurement.exception.approve

procurement.audit.view
```

Exact permission naming shall conform to platform authorization conventions.

---

## 38.3 Segregation Rules

Institutions may configure restrictions such as:

```text
Requester ? Final Approver

Evaluator ? Supplier Selection Approver

Procurement Officer ? Exception Approver
```

For higher-control procurement:

```text
Bid Recorder
      ?
Bid Opener
      ?
Evaluator
      ?
Award Approver
```

These shall be policy-configurable.

---

## 38.4 Delegation

Approval authority may be delegated.

Delegation shall include:

- Delegator
- Delegate
- Authority Scope
- Effective From
- Effective Until
- Reason / Reference
- Audit Record

Expired delegation shall not remain valid.

---

## 38.5 Restricted Procurement

Sensitive procurement may use additional restrictions.

Examples may include:

- Examination-Controlled Procurement
- Security-Sensitive Procurement
- Confidential Institutional Projects
- Other Policy-Classified Procurement

Restricted classification shall affect visibility without requiring a separate procurement architecture.

---

# 39. Configuration Architecture

## 39.1 Principle

Institution-specific procurement behaviour shall be expressed through controlled configuration rather than source-code forks.

---

## 39.2 Procurement Policy Configuration

Institutions may configure:

- Allowed Procurement Methods
- Value Bands
- Minimum Quotation Counts
- Approval Authorities
- Committee Requirements
- Evaluation Methods
- Direct Purchase Eligibility
- Single-Source Rules
- Emergency Procurement Rules
- Negotiation Permission
- Late Submission Rules
- Rate Contract Rules
- External Procurement Rules

---

## 39.3 Threshold Configuration

Thresholds shall be:

- Institution-Scoped
- Effective-Dated
- Versioned
- Auditable

Where applicable, thresholds may differ by:

- Procurement Category
- Funding Context
- Campus
- Procurement Type

---

## 39.4 Workflow Configuration

An institution may configure workflows such as:

```text
Simple Purchase
Requirement ? Approval ? Supplier ? J04
```

or:

```text
Formal Procurement
Requirement
   ?
Tender
   ?
Opening
   ?
Technical Evaluation
   ?
Commercial Evaluation
   ?
Committee
   ?
Approval
   ?
J04
```

Optional stages shall not create artificial completed records when they were never required.

---

## 39.5 Configuration Versioning

A sourcing event shall retain the relevant configuration/policy version used for its governance.

Subsequent configuration changes shall apply according to defined effective-date rules and shall not silently rewrite completed procurement history.

---

# 40. Audit & Traceability

## 40.1 Audit Principle

J01 is a high-audit enterprise domain.

Significant procurement actions shall be attributable and reconstructable.

---

## 40.2 Audit Information

Applicable records shall capture:

- Actor
- Action
- Timestamp
- Institution
- Aggregate
- Previous State where required
- New State
- Reason
- Authority
- Correlation Reference
- Document / Evidence References

---

## 40.3 End-to-End Traceability

J01 shall support tracing:

```text
Procurement Plan
       ¦
       ?
Planned Requirement
       ¦
       ?
Purchase Requisition
       ¦
       ?
Sourcing Event
       ¦
       ?
Supplier Invitations
       ¦
       ?
Supplier Responses
       ¦
       ?
Evaluation
       ¦
       ?
Recommendation
       ¦
       ?
Approval
       ¦
       ?
Approved Sourcing Outcome
       ¦
       ?
Purchase Order Reference
```

---

## 40.4 Evidence Preservation

J01 shall preserve references to applicable:

- Specifications
- Quotations
- Bids
- Proposals
- Comparative Statements
- Evaluation Sheets
- Committee Minutes
- Approval Notes
- Exception Justifications
- External Procurement Evidence

---

## 40.5 Correction

Protected procurement evidence shall not be corrected through direct destructive editing.

Correction shall use controlled actions such as:

- Amendment
- Reopening
- Revised Response
- Revised Evaluation
- Re-Sourcing
- Corrective Administrative Record

The original evidence shall remain available according to retention policy.
# 41. Exception & Reconciliation Architecture

## 41.1 Purpose

Procurement workflows frequently encounter valid operational exceptions.

J01 shall handle such situations through explicit exception states rather than:

- Silent bypasses
- Manual database corrections
- Destructive record changes
- Artificial workflow completion
- Untraceable administrative overrides

---

## 41.2 Exception Categories

Representative J01 exception categories include:

| Exception | Typical Context |
|---|---|
| Insufficient Supplier Responses | Required competition not achieved |
| Procurement Method Override | Recommended method cannot be followed |
| Late Supplier Submission | Response received after deadline |
| Single-Source Exception | Competitive sourcing not applicable |
| Emergency Procurement | Normal sourcing timeline cannot be followed |
| Evaluation Exception | Evaluation cannot proceed normally |
| Committee Exception | Quorum or participation issue |
| External Procurement Exception | External execution differs from expected process |
| Sourcing Cancellation | Procurement process must terminate |
| Re-Sourcing Requirement | Existing sourcing process cannot produce valid outcome |

Institutions may configure additional exception classifications.

---

## 41.3 Exception Lifecycle

A baseline exception lifecycle is:

```text
Exception Detected
       ¦
       ?
Exception Recorded
       ¦
       ?
Supporting Reason / Evidence
       ¦
       ?
Authority Determined
       ¦
       ?
Review
   +---+----+
   ¦        ¦
Approve   Reject
   ¦        ¦
   ?        ?
Controlled  Normal /
Exception   Alternative
Path        Process
   ¦
   ?
Resolved
```

---

## 41.4 Insufficient Supplier Response

Where configured competition requires a minimum number of responses:

```text
Required Responses: N
         ¦
         ?
Actual Valid Responses < N
         ¦
         ?
Competition Exception
         ¦
    +----+-----------+
    ¦    ¦           ¦
Reissue  Invite     Request
         More       Exception
         ¦           ¦
         +-----------+
               ?
        Authorised Decision
```

The architecture shall not universally require procurement cancellation merely because the desired response count was not achieved.

---

## 41.5 Integration Reconciliation

Multi-domain failures may occur after J01 completes a business action.

Example:

```text
ApprovedSourcingOutcomeFinalised
              ¦
              ?
        Event Published
              ¦
              X
        J04 Temporarily
          Unavailable
```

J01 shall not reverse a valid procurement approval merely because downstream integration temporarily fails.

The integration architecture shall support:

- Durable Event Publication
- Retry
- Idempotent Consumption
- Failure Visibility
- Reconciliation
- Administrative Recovery

---

## 41.6 Duplicate Processing

Integration consumers shall use stable identifiers to prevent duplicate downstream consequences.

For example:

```text
Approved Sourcing Outcome
        OutcomeId = X
              ¦
              +----? Delivery Attempt 1
              +----? Delivery Attempt 2

J04 recognises same OutcomeId
              ¦
              ?
One Logical Processing Result
```

---

# 42. Notifications

## 42.1 Ownership

J01 determines **when a procurement notification is required**.

The enterprise notification capability owns delivery through applicable channels.

J01 shall not implement independent email/SMS infrastructure.

---

## 42.2 Representative Notifications

Notifications may be generated for:

- Procurement Plan Submission
- Procurement Plan Approval / Return
- Sourcing Assignment
- Supplier Invitation
- Sourcing Amendment
- Deadline Extension
- Upcoming Submission Deadline
- Evaluation Assignment
- Clarification Request
- Committee Action
- Recommendation Approval
- Recommendation Rejection
- Procurement Exception
- Tender Cancellation
- Award Outcome
- Re-Sourcing

---

## 42.3 Supplier Notifications

Where supported, supplier-facing notifications may include:

- Invitation
- Amendment / Corrigendum
- Deadline Change
- Clarification
- Outcome Notification

Supplier notification shall respect procurement confidentiality.

Unsuccessful suppliers shall not receive another supplier's protected commercial or evaluation information unless an applicable institutional process explicitly permits disclosure.

---

## 42.4 Notification Failure

Notification-delivery failure shall not normally invalidate the underlying procurement transaction.

Where successful notification is legally or institutionally required before progression, workflow policy may place the process into an exception state.

---

# 43. Documents & Evidence

## 43.1 Principle

Procurement produces significant documentary evidence.

J01 owns the business relationship between procurement records and documents but shall use the enterprise document capability for canonical document storage where that capability is defined.

---

## 43.2 Representative Documents

J01 may reference:

- Procurement Plans
- Technical Specifications
- RFQs
- RFPs
- Tender Documents
- Corrigenda
- Supplier Quotations
- Technical Proposals
- Commercial Proposals
- Bid Documents
- Clarification Documents
- Comparative Statements
- Evaluation Sheets
- Committee Minutes
- Proprietary / Single-Source Evidence
- Emergency Procurement Evidence
- External Procurement Documents
- Award Recommendations

---

## 43.3 Document Metadata

Procurement document references should retain applicable:

- Document ID
- Document Type
- Version
- Related Aggregate
- Supplier where applicable
- Confidentiality Classification
- Uploaded / Generated By
- Timestamp
- Integrity Metadata where supported

---

## 43.4 Versioning

Material procurement documents shall support version history.

Example:

```text
Tender Document V1
       ¦
       ?
Corrigendum
       ¦
       ?
Tender Document V2
```

The previous version remains evidence.

---

## 43.5 Confidential Documents

Technical and commercial submissions may require different access restrictions.

The architecture shall support document-level authorization consistent with sourcing-stage authorization.

---

# 44. Multi-Domain Integration

## 44.1 Integration Principle

J01 participates in the broader Enterprise Resource Management lifecycle.

Its integrations shall preserve canonical ownership.

---

## 44.2 J02 — Vendor & Supplier Management

J01 consumes from J02:

- Supplier Identity
- Supplier Status
- Qualification
- Category Eligibility
- Suspension / Blacklisting State
- Supplier Performance where applicable

J01 produces sourcing participation and outcome information that may contribute to future supplier-performance analysis.

```text
J02 Supplier
     ¦
     ?
J01 Sourcing Participation
     ¦
     ?
Procurement Outcome
     ¦
     +--------? J02 Performance Context
```

J01 shall not modify Supplier Master records directly.

---

## 44.3 J03 — Purchase Requisition & Approval

J03 provides authorised procurement demand.

```text
Requesting Domain
       ¦
       ?
      J03
Purchase Requisition
       ¦
       ?
Approved Demand
       ¦
       ?
      J01
```

J01 shall preserve requisition references throughout sourcing.

---

## 44.4 J04 — Purchase Order Management

J01 produces approved sourcing outcomes.

```text
J01
Evaluation
   ?
Recommendation
   ?
Approval
   ?
Approved Sourcing Outcome
   ¦
   ?
J04
Purchase Order
```

J04 shall validate that the downstream commitment remains consistent with the approved sourcing outcome.

---

## 44.5 J05 — Goods Receipt, Inspection & Acceptance

J01 does not normally interact directly with Goods Receipt for transactional execution.

The primary lifecycle is:

```text
J01 Sourcing
     ?
J04 Purchase Order
     ?
J05 Goods Receipt
```

J01 procurement information may remain available for traceability and reporting.

---

## 44.6 J06 — Inventory & Warehouse Management

J01 may reference J06 Item Master entries during procurement.

```text
J06 Item
   ¦
   ?
J01 Sourcing Line
```

Procurement of a new resource may precede formal Item Master creation where institutional policy permits.

J06 remains canonical owner of inventory identity and stock.

---

## 44.7 J08 — Asset Lifecycle Management

J01 may identify that a procurement requirement concerns a potential asset category.

Actual operational asset registration occurs downstream after applicable receipt and acceptance.

J01 shall not create operational assets merely because a supplier was selected.

---

## 44.8 Finance

Finance integration may support:

- Budget Validation
- Funding References
- Financial Approval Context
- Procurement Value Controls
- Downstream Accounts Payable / Payment through later lifecycle stages

The ownership rule remains:

> **J01 owns sourcing truth; Finance owns monetary and accounting truth.**

---

## 44.9 Requesting Domains

Any authorised institutional domain may initiate procurement demand through the appropriate procurement-demand architecture.

Representative consumers may include:

```text
Academic Operations -------+
People & Workforce --------¦
Campus Operations ---------¦
Examinations --------------¦
Administration ------------+--? J03 ? J01
Research ------------------¦
Library -------------------¦
Hostel --------------------¦
Medical / Security --------+
```

The requesting domain owns the operational reason for procurement.

J01 owns the sourcing process.

---

# 45. Reporting & KPIs

## 45.1 Procurement Planning Reports

J01 may provide:

- Planned Procurement by Period
- Planned Procurement by Campus
- Planned Procurement by Department
- Planned versus Actual Procurement
- Unplanned Procurement
- Procurement Plan Completion

---

## 45.2 Sourcing Reports

Representative reports include:

- Sourcing Events by Status
- Procurement by Method
- Open Sourcing Events
- Delayed Sourcing Events
- Cancelled Sourcing
- Re-Sourcing Cases
- No-Response Cases
- No-Suitable-Bid Cases

---

## 45.3 Competition Reports

J01 may report:

- Suppliers Invited
- Responses Received
- Average Response Count
- Insufficient Competition Cases
- Single-Source Procurement
- Direct Procurement
- Competitive Procurement Share

These metrics shall support governance review without automatically asserting wrongdoing.

---

## 45.4 Cycle-Time Metrics

Possible metrics include:

```text
Demand Approval ? Sourcing Start

Sourcing Start ? Publication

Publication ? Submission Close

Submission Close ? Evaluation Complete

Evaluation Complete ? Recommendation

Recommendation ? Approval

Overall Procurement Sourcing Cycle Time
```

---

## 45.5 Evaluation Reports

J01 may provide:

- Technical Qualification Outcomes
- Commercial Evaluation Status
- Pending Evaluations
- Evaluation Turnaround Time
- Supplier Selection Basis
- Non-Lowest Selection Cases

---

## 45.6 Exception Reports

Governance reports may include:

- Emergency Procurement
- Single-Source Procurement
- Procurement Method Overrides
- Late Bid Exceptions
- Insufficient Quotation Exceptions
- Committee Exceptions
- Repeated Exception Patterns

---

## 45.7 External Procurement Reports

J01 may report:

- Externally Executed Procurement
- External Platform / Agency
- Pending External Outcomes
- Pending Internal Recognition / Approval
- External Procurement Cycle Time

---

# 46. Future Extension Points

## 46.1 Supplier Portal

A future Supplier Portal may allow suppliers to:

- View Invitations
- Accept / Decline Participation
- Submit Quotations
- Submit Proposals
- Upload Documents
- Respond to Clarifications
- Submit Revised Offers
- View Permitted Outcome Information

J01 architecture shall not depend on the portal being available.

---

## 46.2 E-Tender Integration

Future adapters may integrate with applicable external e-procurement platforms.

Integration shall be implemented through adapters/contracts rather than embedding external-platform semantics throughout the J01 domain model.

---

## 46.3 Digital Signatures

Future digital-signature capability may support:

- Tender Approval
- Committee Recommendation
- Evaluation Finalisation
- Award Approval
- Procurement Documents

Digital signatures shall augment authorization and evidence rather than replace domain-state validation.

---

## 46.4 AI-Assisted Procurement

Future AI capabilities may assist with:

- Requirement Consolidation Suggestions
- Supplier Comparison
- Evaluation Summaries
- Procurement Cycle Analysis
- Historical Price Analysis
- Unusual Quotation Pattern Detection
- Repeated Emergency Procurement Detection
- Procurement Planning Forecasts

AI outputs shall remain advisory unless a separately approved architecture explicitly establishes otherwise.

AI shall not autonomously:

- Approve procurement.
- Disqualify suppliers.
- Select suppliers.
- Override procurement policy.
- Create binding purchasing commitments.

---

## 46.5 Market Benchmarking

Future external data integrations may provide market-price or category benchmarks.

Benchmark information shall be treated as decision support rather than canonical supplier quotation data.

---

# 47. Architectural Constraints

J01 shall comply with the following constraints:

1. J01 shall not own Supplier Master.
2. J01 shall not own Purchase Requisition.
3. J01 shall not own Purchase Order.
4. J01 shall not own Goods Receipt.
5. J01 shall not own Stock Ledger.
6. J01 shall not own operational Asset Register.
7. J01 shall not own budget balances or accounting entries.
8. J01 shall not hard-code one institution's procurement thresholds.
9. J01 shall not assume every procurement requires tendering.
10. J01 shall not assume the lowest price must win.
11. J01 shall not silently bypass configured competition requirements.
12. J01 shall not destructively rewrite submitted supplier responses.
13. J01 shall not fabricate internal execution for externally performed procurement.
14. J01 shall preserve sourcing and evaluation history.
15. J01 shall support institution-configurable procurement complexity.
16. J01 shall support multi-campus authorization where required.
17. J01 shall support controlled procurement exceptions.
18. J01 shall use multi-domain contracts rather than multi-domain database ownership.

---

# 48. Governance Decisions

The following decisions are canonical for J01.

### GD-J01-01 — Public and Private Institution Support

J01 shall support both private and government/public higher-education institutions through configurable procurement policy.

### GD-J01-02 — Simplified Procurement

Low-complexity procurement shall be supported without forcing formal tender workflows.

### GD-J01-03 — Committee Procurement

Purchase, technical, evaluation, and tender committee participation shall be configurable.

### GD-J01-04 — Institution-Specific Procurement Rules

Thresholds, quotation counts, approval authority, procurement methods, and workflow requirements shall be configurable.

### GD-J01-05 — External Procurement

J01 shall support procurement performed partly or completely outside SutraERP.

### GD-J01-06 — Supplier Ownership

J02 remains canonical owner of supplier identity and supplier eligibility.

### GD-J01-07 — Procurement Demand Ownership

J03 remains canonical owner of Purchase Requisition and authorised procurement demand.

### GD-J01-08 — Purchase Order Ownership

J04 remains canonical owner of Purchase Orders.

### GD-J01-09 — Financial Ownership

Finance remains canonical owner of budget, accounting, payables, and payment.

### GD-J01-10 — Selection Principle

Lowest price shall not universally equal supplier selection.

### GD-J01-11 — Exception Governance

Procurement exceptions shall be explicit, authorised where required, and auditable.

### GD-J01-12 — Historical Integrity

Submitted bids, formal evaluations, approvals, and sourcing outcomes shall not be destructively rewritten.

### GD-J01-13 — External Execution Integrity

J01 shall distinguish activities executed in SutraERP from activities merely recorded from external procurement systems.

### GD-J01-14 — AI Boundary

Future AI capabilities shall remain advisory for supplier evaluation and procurement decisions unless explicitly governed otherwise.

---

# 49. Workflow Catalogue

The canonical J01 workflow set is:

| Workflow ID | Workflow |
|---|---|
| J01-W01 | Procurement Plan Creation |
| J01-W02 | Procurement Plan Review & Approval |
| J01-W03 | Procurement Plan Revision |
| J01-W04 | Procurement Demand Intake |
| J01-W05 | Procurement Demand Consolidation |
| J01-W06 | Procurement Method Determination |
| J01-W07 | Procurement Method Override |
| J01-W08 | Simplified / Direct Procurement |
| J01-W09 | RFQ Preparation & Issue |
| J01-W10 | Supplier Invitation |
| J01-W11 | Supplier Quotation / Response Submission |
| J01-W12 | RFQ Amendment |
| J01-W13 | RFP Procurement |
| J01-W14 | Tender Preparation & Publication |
| J01-W15 | Tender Corrigendum |
| J01-W16 | Bid Receipt & Submission Closure |
| J01-W17 | Formal Bid Opening |
| J01-W18 | Technical Evaluation |
| J01-W19 | Technical Clarification |
| J01-W20 | Commercial Evaluation |
| J01-W21 | Comparative Statement |
| J01-W22 | Supplier Clarification |
| J01-W23 | Negotiation |
| J01-W24 | Best and Final Offer |
| J01-W25 | Supplier Recommendation |
| J01-W26 | Split Award Recommendation |
| J01-W27 | Award Approval |
| J01-W28 | Procurement / Tender Committee Decision |
| J01-W29 | Rate Contract Establishment |
| J01-W30 | Procurement Against Rate Contract |
| J01-W31 | Single-Source Procurement |
| J01-W32 | Emergency Procurement |
| J01-W33 | Emergency Procurement Post-Review |
| J01-W34 | External Procurement Registration |
| J01-W35 | External Procurement Outcome Recognition |
| J01-W36 | Sourcing Cancellation |
| J01-W37 | No-Response Closure |
| J01-W38 | No-Suitable-Bid Closure |
| J01-W39 | Re-Sourcing |
| J01-W40 | Procurement Exception Resolution |

---

# 50. Dependency Matrix

| Domain / Capability | Relationship with J01 | Ownership |
|---|---|---|
| J00 | Governs Section J boundaries | J00 |
| J02 | Supplier identity, eligibility and performance | J02 |
| J03 | Purchase Requisition / authorised demand | J03 |
| J04 | Purchase Order creation and management | J04 |
| J05 | Downstream receipt and inspection | J05 |
| J06 | Item Master references | J06 |
| J08 | Downstream operational asset lifecycle | J08 |
| Finance | Budget and monetary truth | Finance |
| Requesting Domains | Operational requirement/specification | Respective Domain |
| Identity / Authorization | Actor identity and permissions | Platform / Canonical Identity Domain |
| Workflow | Workflow execution infrastructure | Platform |
| Documents | Enterprise document storage | Canonical Document Capability |
| Notifications | Delivery infrastructure | Enterprise Notification Capability |
| Audit | Enterprise audit infrastructure | Platform / Governance Capability |

---

# 51. Completion Criteria

J01 architecture shall be considered complete when implementation can support the following without violating ownership boundaries:

```text
Procurement Need
      ¦
      ?
Approved Demand
      ¦
      ?
Procurement Method
      ¦
      +-- Direct / Simplified
      ¦
      +-- RFQ
      ¦
      +-- RFP
      ¦
      +-- Tender
      ¦
      +-- Rate Contract
      ¦
      +-- Single Source
      ¦
      +-- Emergency
      ¦
      +-- External Procurement
      ¦
      ?
Supplier Participation
      ¦
      ?
Evaluation
      ¦
      ?
Recommendation
      ¦
      ?
Approval
      ¦
      ?
Approved Sourcing Outcome
      ¦
      ?
J04 Purchase Order Management
```

The implementation shall additionally demonstrate:

- Configurable procurement methods and thresholds.
- Support for both simple colleges and formal university procurement.
- Public/private institution configurability.
- Multi-campus procurement authorization.
- Supplier integration through J02.
- Demand traceability to J03.
- Controlled quotation and tender workflows.
- Technical and commercial evaluation.
- Committee-based procurement where configured.
- Direct, single-source, and emergency procurement.
- External procurement tracking.
- Rate-contract support.
- Supplier-response versioning.
- Controlled exceptions.
- Complete auditability.
- Non-destructive historical evidence.
- Secure access to confidential supplier information.
- Reliable publication of approved sourcing outcomes for J04.
- No leakage of Finance, supplier, inventory, asset, or Purchase Order ownership into J01.

---

# J01 Canonical Boundary Summary

```text
                         J01
          Procurement Planning & Sourcing

                           OWNS
                             ¦
       +---------------------+---------------------+
       ¦                     ¦                     ¦
       ?                     ?                     ?
 Procurement Planning    Sourcing             Evaluation
       ¦                     ¦                     ¦
       +---------------------+---------------------+
                             ¦
                             ?
                    Supplier Recommendation
                             ¦
                             ?
                   Approved Sourcing Outcome

                           DOES NOT OWN
                             ¦
       +---------------------+---------------------+
       ?          ?          ?          ?          ?
    Supplier   Purchase    Purchase    Budget    Inventory
     Master   Requisition   Order     /Payment    /Assets
       ¦          ¦          ¦          ¦          ¦
       ?          ?          ?          ?          ?
      J02        J03        J04      Finance    J06/J08
```

> **J01 determines how an authorised institutional procurement requirement is sourced, evaluated, and converted into an approved supplier-selection outcome. It does not own the supplier, the originating demand, the resulting Purchase Order, the money, or the physical resource lifecycle.**
