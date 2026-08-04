# L2-J10 — Disposal, Scrap & Write-Off

## Document Metadata

| Attribute | Value |
|---|---|
| Document ID | L2-J10 |
| Document Name | Disposal, Scrap & Write-Off |
| Domain | Procurement & Inventory |
| Architecture Level | Level 2 |
| Document Type | Domain Workflow Architecture |
| Bounded Context | Disposal, Scrap & Write-Off |
| Document Owner | Chetan |
| Architectural Style | Domain-Driven Design (DDD), CQRS, Event-Driven Architecture |
| Primary Aggregate | Disposal & Write-off Case |
| Status | Draft |
| Version | 1.0 |

---

# 1. Purpose

The Disposal, Scrap & Write-Off domain governs the controlled retirement, disposal, destruction, sale, donation, transfer-out, scrap processing, and write-off of institution-owned inventory within SutraERP.

J10 provides the governed workflow required to:

- Identify inventory requiring disposal or write-off
- Create disposal/write-off cases
- Classify disposal reasons
- Establish disposal scope
- Preserve item and quantity evidence
- Obtain condition and inspection evidence
- Record valuation references
- Conduct committee or authority review
- Obtain required approvals
- Select an authorized disposal method
- Execute physical disposal
- Record destruction or handover evidence
- Record sale/auction outcomes where applicable
- Record scrap outcomes
- Record donation or transfer-out outcomes
- Govern inventory-loss write-off
- Request canonical inventory adjustment
- Coordinate asset retirement where applicable
- Coordinate financial accounting consequences
- Reconcile approved and executed quantities
- Complete and close disposal/write-off cases
- Preserve immutable institutional evidence

J10 acts as the enterprise system of record for the **disposal/write-off business case, authorization lifecycle, execution evidence, and disposal outcome**.

J10 does not become the authoritative system of record for inventory quantities, asset records, or accounting balances.

Canonical inventory state remains owned by **J06 — Inventory & Warehouse Management**.

Canonical asset lifecycle state remains owned by **J08 — Asset Lifecycle Management** where the disposed item represents or is linked to an institutional asset.

Inventory audit findings remain owned by **J09 — Physical Verification, Audit & Reconciliation**.

Financial accounting and ledger consequences remain owned by the applicable Finance domain.

The architecture follows:

- Domain-Driven Design
- CQRS
- Event-Driven Architecture
- Explicit aggregate boundaries
- Immutable business evidence
- Segregation of duties
- Canonical ownership
- Reliable multi-domain integration
- Institution-aware multi-tenant isolation

---

# 2. Enterprise Context

Disposal, Scrap & Write-Off operates at the terminal inventory-governance stage of the SutraERP Procurement & Inventory architecture.

Institutional inventory may become unsuitable for continued operational use because it is:

- Damaged
- Broken
- Obsolete
- Unserviceable
- Expired where applicable
- Surplus
- Technologically outdated
- Economically uneconomical to repair
- Lost
- Missing
- Destroyed
- Contaminated or unsafe where applicable
- No longer institutionally required
- Recommended for retirement through inventory audit
- Otherwise eligible under institutional disposal policy

J10 governs the institutional decision and execution process required to remove such inventory from active institutional custody through an approved disposal or write-off mechanism.

---

## 2.1 Typical Enterprise Flow

```text
Inventory & Warehouse Management (J06)
          ¦
          ¦ Inventory / Item Reference
          ?
Disposal Requirement Identified
          ¦
          +--------------? Inventory Audit (J09)
          ¦                Audit Finding / Variance
          ¦
          +--------------? Asset Lifecycle Management (J08)
          ¦                Asset Retirement Requirement
          ¦
          ?
Disposal / Write-off Case
          ¦
          ?
Eligibility Assessment
          ¦
          ?
Condition / Evidence Review
          ¦
          ?
Valuation Reference
          ¦
          ?
Committee / Authority Review
          ¦
          ?
Approval
          ¦
          ?
Approved Disposal Method
          ¦
          ?
Physical Disposal / Write-off Execution
          ¦
          ?
Execution Evidence
          ¦
          ?
Inventory Adjustment Request
          ¦
          ?
Inventory & Warehouse Management (J06)
          ¦
          ?
Financial / Asset Consequences
          ¦
          ?
Reconciliation
          ¦
          ?
Case Completion & Closure
```

J10 records why disposal/write-off was proposed, how it was authorized, how it was executed, and what institutional evidence proves the outcome.

J06 remains responsible for authoritative inventory-state modification.

---

# 3. Domain Scope

## 3.1 In Scope

The Disposal, Scrap & Write-Off domain governs:

- Disposal Request
- Write-off Request
- Disposal & Write-off Case
- Disposal Case Number
- Disposal Case Lines
- Inventory Item References
- Quantity Proposed for Disposal
- Quantity Proposed for Write-off
- Disposal Reason
- Write-off Reason
- Disposal Classification
- Condition Assessment
- Supporting Inspection Evidence
- Audit Finding References
- Loss/Damage Evidence
- Disposal Eligibility Assessment
- Disposal Recommendation
- Disposal Method Selection
- Valuation Reference
- Residual Value Reference
- Scrap Value Reference
- Committee Review
- Committee Recommendation
- Approval State
- Approval History
- Disposal Authorization
- Write-off Authorization
- Disposal Execution
- Destruction Execution
- Scrap Handover
- Sale/Auction Outcome
- Donation Handover
- External Transfer-out Evidence
- Disposal Recipient Reference
- Disposal Certificate
- Destruction Certificate
- Handover Evidence
- Execution Witnesses
- Disposal Quantity Reconciliation
- Write-off Quantity Reconciliation
- Inventory Adjustment Request Reference
- Asset Retirement Reference
- Financial Consequence Reference
- Disposal Exception
- Disposal Cancellation
- Disposal Completion
- Disposal Closure
- Disposal Archival
- Disposal Audit Evidence

---

## 3.2 Out of Scope

The following capabilities remain outside J10:

| Capability | Canonical Owner |
|---|---|
| Inventory Item Master | Inventory & Warehouse Management (J06) |
| Inventory Stock Balance | Inventory & Warehouse Management (J06) |
| Inventory Reservation | Inventory & Warehouse Management (J06) |
| Inventory Adjustment Execution | Inventory & Warehouse Management (J06) |
| Inventory Valuation Ledger | Inventory Management / Finance |
| Asset Master | Asset Lifecycle Management (J08) |
| Asset Depreciation | Asset Lifecycle / Finance |
| Asset Accounting | Finance / Asset Lifecycle |
| Stock Transfer Execution | Stock Operations & Distribution (J07) |
| Inventory Audit | Physical Verification, Audit & Reconciliation (J09) |
| Physical Count Evidence | Physical Verification, Audit & Reconciliation (J09) |
| Audit Variance | Physical Verification, Audit & Reconciliation (J09) |
| Purchase Order | Purchase Order Management (J04) |
| Supplier Goods Receipt | Goods Receipt, Inspection & Acceptance (J05) |
| Vendor Master | Vendor & Supplier Management (J02) |
| General Ledger | Finance |
| Financial Journal Posting | Finance |
| Payment Processing | Finance |
| Authentication | Identity Management |
| Authorization Policy | Authorization Domain |
| Workflow Routing | Workflow Engine |
| Enterprise Notifications | Notification Infrastructure |
| Enterprise Audit Repository | Audit Domain |

J10 may consume or reference these capabilities but shall not duplicate their canonical state.

---

# 4. Canonical Ownership

## 4.1 J10 Canonically Owns

The Disposal & Write-off bounded context is the canonical owner of:

- Disposal & Write-off Case
- Disposal Case Header
- Disposal Case Lines
- Disposal Case Lifecycle
- Disposal Case Status
- Disposal Classification
- Disposal Reason
- Write-off Reason
- Disposal Eligibility Determination
- Condition Evidence Reference
- Disposal Recommendation
- Disposal Method Decision
- Disposal Committee Review Record
- Disposal Committee Recommendation
- Disposal Approval State
- Disposal Authorization Record
- Write-off Authorization Record
- Disposal Execution Record
- Disposal Execution Lines
- Disposal Recipient Reference
- Disposal Handover Record
- Destruction Record
- Scrap Disposal Record
- Auction/Sale Outcome Reference
- Donation Outcome Record
- Disposal Certificate
- Destruction Certificate Reference
- Disposal Witness Evidence
- Disposal Exception
- Disposal Reconciliation State
- Disposal Completion State
- Disposal Closure State
- Disposal Cancellation State
- Disposal Audit Metadata

---

## 4.2 J10 Does Not Canonically Own

J10 does not own:

- Inventory Item
- Inventory Quantity
- Available Quantity
- Reserved Quantity
- Inventory Cost Ledger
- Inventory Valuation Ledger
- Warehouse Master
- Store Master
- Campus Master
- Department Master
- Asset Master
- Asset Book Value
- Asset Depreciation
- Inventory Audit
- Physical Count
- Audit Variance
- Stock Transfer
- Financial Journal
- General Ledger Balance
- Vendor Master

These records shall be referenced using stable enterprise identifiers.

Where historical integrity requires preservation of transaction-time information, J10 may store controlled snapshots such as:

- Item Description Snapshot
- Item Classification Snapshot
- Unit of Measure Snapshot
- Location Snapshot
- Condition Snapshot
- Valuation Reference Snapshot
- Asset Reference Snapshot
- Custodian Snapshot

Such snapshots provide historical evidence and shall not become competing master records.

---

# 5. Critical Ownership Boundaries

## 5.1 J10 and J06 — Inventory & Warehouse Management

The separation between **J10 — Disposal, Scrap & Write-Off** and **J06 — Inventory & Warehouse Management** is mandatory.

J10 owns:

> **Why inventory is being disposed/written off, who authorized it, how disposal occurred, and what evidence proves the outcome.**

J06 owns:

> **The authoritative quantity and inventory state before and after the approved inventory adjustment.**

J10 shall therefore never directly modify canonical stock balances.

---

## 5.2 Example — Physical Disposal

An institution approves disposal of 25 damaged chairs.

J10 records:

```text
Disposal Case:
DW-000245

Item:
Chair — Standard Classroom

Reason:
Damaged Beyond Economical Repair

Quantity Proposed:
25

Quantity Approved:
25

Disposal Method:
Scrap

Quantity Physically Disposed:
25

Execution Status:
Completed

Evidence:
Scrap Handover Record
Witness Confirmation
Disposal Certificate
```

J10 then requests the appropriate inventory-state adjustment from J06.

J06 remains responsible for changing the authoritative inventory quantity/state.

---

## 5.3 J10 and J08 — Asset Lifecycle Management

Where inventory proposed for disposal is linked to an institutional asset, J10 shall not independently retire, derecognize, or alter the canonical asset record.

The ownership boundary is:

```text
J10
=
Disposal / Write-off Authorization
+
Physical Disposal Evidence
+
Disposal Outcome
```

```text
J08
=
Canonical Asset Lifecycle
+
Asset Retirement State
+
Asset Disposal Linkage
```

J10 shall provide disposal outcome facts to J08 through governed integration.

---

## 5.4 J10 and J09 — Physical Verification, Audit & Reconciliation

J09 may identify:

- Missing inventory
- Damaged inventory
- Obsolete inventory
- Unusable inventory
- Unexplained shortage
- Stock requiring investigation
- Stock recommended for write-off

J09 owns the audit finding.

J10 owns the subsequent disposal/write-off case when institutional policy requires formal disposal/write-off processing.

The relationship is:

```text
Inventory Audit (J09)
        ¦
        ¦ Audit Finding / Recommendation
        ?
Disposal, Scrap & Write-Off (J10)
        ¦
        ¦ Governed Disposal / Write-off Process
        ?
Approved Outcome
```

J10 shall not modify J09 audit evidence.

---

## 5.5 J10 and Finance

J10 may provide approved disposal/write-off facts required for financial processing.

J10 shall not independently:

- Post journals
- Modify general-ledger balances
- Recognize accounting loss
- Calculate authoritative depreciation
- Derecognize financial assets
- Recognize sale proceeds
- Perform financial settlement

These remain owned by Finance and other applicable financial bounded contexts.

---

# 6. Core Business Concepts

## 6.1 Disposal & Write-off Case

A Disposal & Write-off Case represents the governed institutional process through which inventory is evaluated and authorized for disposal, write-off, or another approved terminal disposition.

The case provides the primary business identity for the complete lifecycle.

A case may contain one or more Disposal Case Lines.

---

## 6.2 Disposal Request

A Disposal Request represents a proposal to remove institution-owned inventory from operational custody through an approved disposal method.

A request may originate from:

- Warehouse personnel
- Department personnel
- Inventory management
- Asset management
- Inventory audit
- Institutional inspection
- Authorized committee
- Other governed institutional process

Creation of a Disposal Request does not itself authorize disposal.

---

## 6.3 Write-off Request

A Write-off Request represents a proposal to formally recognize that inventory can no longer remain represented as usable institutional stock because of an approved business reason.

Examples may include:

- Confirmed loss
- Irrecoverable shortage
- Destruction
- Damage beyond recovery
- Missing stock after completed investigation
- Expiry where applicable
- Other institutionally approved reasons

A Write-off Request is not itself an inventory adjustment.

J06 executes the canonical inventory adjustment after the required J10 authorization.

---

## 6.4 Disposal Case Line

A Disposal Case Line represents a specific inventory item and quantity being evaluated within a Disposal & Write-off Case.

Each line may preserve:

- Inventory Item Reference
- Item Description Snapshot
- Inventory Classification
- Unit of Measure
- Location Reference
- Custodian Reference
- Asset Reference where applicable
- Quantity Proposed
- Quantity Eligible
- Quantity Approved
- Quantity Executed
- Quantity Rejected
- Quantity Pending
- Condition
- Disposal Reason
- Approved Disposal Method
- Valuation Reference
- Line Status

Line-level processing allows a case to contain items that receive different decisions.

---

## 6.5 Disposal Eligibility

Disposal Eligibility represents the governed determination that an item or quantity may proceed through disposal/write-off processing.

Eligibility may consider:

- Item condition
- Operational usefulness
- Repair feasibility
- Remaining institutional need
- Age
- Obsolescence
- Expiry where applicable
- Safety status
- Audit findings
- Inspection findings
- Asset status
- Inventory status
- Applicable institutional policy
- Required supporting evidence

Eligibility does not itself constitute final approval.

---

## 6.6 Condition Assessment

Condition Assessment records the observed state of inventory proposed for disposal.

Representative conditions may include:

- Serviceable but Surplus
- Damaged
- Unserviceable
- Beyond Economical Repair
- Obsolete
- Expired
- Unsafe
- Destroyed
- Missing
- Lost
- Scrap
- Other Institution-Configured Condition

Condition assessment evidence shall remain traceable to its source.

---

## 6.7 Disposal Recommendation

A Disposal Recommendation represents the proposed institutional treatment after eligibility and evidence review.

Recommendations may include:

- Continue in Service
- Repair
- Reallocate
- Return to Inventory
- Transfer Internally
- Sell
- Auction
- Scrap
- Destroy
- Donate
- Transfer Out
- Write Off
- Reject Disposal Request
- Require Further Investigation

Recommendations do not modify inventory state.

---

## 6.8 Disposal Method

Disposal Method identifies the authorized mechanism through which inventory leaves active institutional custody.

Representative methods include:

- Scrap
- Destruction
- Sale
- Auction
- Donation
- External Transfer
- Authorized Recycling
- Return to Authorized External Party
- Other Institution-Approved Method

The permitted methods shall be governed by institutional configuration and policy.

---

## 6.9 Write-off

Write-off represents the governed institutional authorization that inventory shall no longer remain represented as active usable inventory because of an approved reason.

Write-off authorization is a J10 business fact.

Canonical quantity adjustment remains a J06 responsibility.

Financial write-off accounting remains a Finance responsibility.

---

## 6.10 Disposal Committee Review

Disposal Committee Review represents formal evaluation by an institutionally authorized group where policy requires collective review.

The review may record:

- Committee Reference
- Meeting Reference
- Review Date
- Members
- Quorum Evidence
- Reviewed Case Lines
- Findings
- Recommendations
- Dissenting Remarks where applicable
- Supporting Evidence
- Decision Reference

Committee configuration and workflow routing remain governed externally where applicable.

---

## 6.11 Disposal Authorization

Disposal Authorization represents the approved institutional permission to execute a specific disposal/write-off outcome.

Authorization shall identify:

- Case
- Applicable Lines
- Approved Quantity
- Approved Disposal Method
- Approval Authority
- Approval Timestamp
- Conditions
- Validity Period where applicable
- Approval Reference

Authorization shall be immutable once used for execution.

---

## 6.12 Disposal Execution

Disposal Execution represents the physical implementation of an approved disposal decision.

Execution may involve:

```text
Approved Inventory
        ¦
        ?
Physical Verification
        ¦
        ?
Authorized Handover / Destruction
        ¦
        ?
Witness Confirmation
        ¦
        ?
Supporting Evidence
        ¦
        ?
Execution Confirmation
```

Execution shall not exceed the applicable approved quantity.

---

## 6.13 Disposal Evidence

Disposal Evidence represents the documentary and operational proof that an authorized disposal activity occurred.

Evidence may include:

- Disposal Certificate
- Destruction Certificate
- Handover Record
- Recipient Acknowledgement
- Scrap Receipt
- Auction/Sale Reference
- Donation Acknowledgement
- Witness Confirmation
- Inspection Record
- Supporting Document Reference
- Approved digital evidence

Evidence provenance shall remain traceable.

---

## 6.14 Disposal Recipient

A Disposal Recipient represents an external or internal party receiving inventory through an authorized disposal method.

J10 may record a recipient reference and transaction-time snapshot.

J10 shall not automatically become the canonical owner of vendor, organization, alumni, employee, or other enterprise party master data.

---

## 6.15 Disposal Reconciliation

Disposal Reconciliation confirms that:

- Approved quantities
- Physically executed quantities
- Inventory adjustment quantities
- Applicable asset outcomes
- Applicable financial references

have reached an acceptable governed outcome.

A case shall not be considered fully complete merely because physical disposal occurred.

---

# 7. Disposal & Write-off Types

J10 supports classifications such as:

- Standard Disposal
- Damaged Inventory Disposal
- Obsolete Inventory Disposal
- Surplus Inventory Disposal
- Expired Inventory Disposal
- Scrap Disposal
- Destruction
- Sale Disposal
- Auction Disposal
- Donation
- External Transfer-out
- Recycling Disposal
- Loss Write-off
- Missing Inventory Write-off
- Irrecoverable Shortage Write-off
- Destroyed Inventory Write-off
- Audit-Recommended Write-off
- Asset-Linked Disposal
- Emergency Disposal

Institutions may configure additional classifications where required.

Classification shall not bypass authorization or evidence requirements.

---

# 8. Disposal Priority & Materiality

## 8.1 Priority

Representative disposal priorities may include:

- Low
- Normal
- High
- Critical
- Emergency

Priority may influence:

- Review scheduling
- Workflow SLA
- Committee scheduling
- Approval urgency
- Execution scheduling
- Notification urgency

Priority shall not silently bypass institutional authorization.

---

## 8.2 Materiality

Disposal/write-off materiality may be determined using institutionally governed criteria such as:

- Quantity
- Inventory category
- Estimated value
- Book-value reference
- Replacement significance
- Safety impact
- Regulatory sensitivity
- Asset classification
- Operational criticality

Materiality may influence:

- Required approval authority
- Committee requirement
- Independent inspection requirement
- Witness requirement
- Financial review requirement
- Evidence requirement

J10 shall consume authoritative valuation or accounting references where required rather than becoming a competing financial valuation system.

---

# 9. Canonical Aggregate Model

## 9.1 Aggregate Design Principles

J10 follows Domain-Driven Design and establishes explicit transactional consistency boundaries.

The aggregate model shall ensure that:

- Disposal/write-off decisions remain internally consistent.
- Unauthorized disposal cannot be executed.
- Execution cannot exceed approved scope.
- Historical approvals remain immutable.
- Physical disposal evidence remains immutable.
- Multi-domain ownership boundaries remain intact.
- External aggregates are never directly modified.
- Concurrent operations cannot silently corrupt disposal state.
- Every material lifecycle transition remains traceable.
- Disposal execution and inventory adjustment remain distinguishable business facts.

---

## 9.2 Root Aggregate — Disposal & Write-off Case

The **Disposal & Write-off Case** is the primary aggregate root of J10.

It governs the complete lifecycle from initial proposal through final closure.

---

### Aggregate Identity

Every Disposal & Write-off Case shall have an immutable:

- Tenant Identifier
- Institution Identifier
- Disposal Case Identifier
- Disposal Case Number

The Disposal Case Number may follow an institution-configurable numbering policy.

---

### Aggregate Responsibilities

The root aggregate governs:

- Case identity
- Case type
- Case reason
- Priority
- Materiality
- Originating reference
- Disposal Case Lines
- Eligibility state
- Assessment state
- Recommendation state
- Committee-review state
- Approval state
- Authorization state
- Execution state
- Reconciliation state
- Cancellation state
- Completion state
- Closure state
- Audit metadata

---

### Aggregate Invariants

The Disposal & Write-off Case shall enforce:

1. Every case belongs to exactly one tenant and institution.

2. Every case shall contain at least one valid Disposal Case Line before submission.

3. Case identity is immutable.

4. Proposed quantities shall be greater than zero.

5. A case cannot execute disposal before required approval is complete.

6. Execution shall not exceed authorized quantity.

7. Disposal method used during execution shall match the authorized method unless a governed amendment is approved before execution.

8. Confirmed execution evidence cannot be silently altered.

9. Inventory adjustment confirmation shall not be fabricated by J10.

10. Asset retirement confirmation shall not be fabricated by J10.

11. Financial posting confirmation shall not be fabricated by J10.

12. Completed cases cannot accept new disposal execution.

13. Closed cases are read-only.

14. Archived cases are permanently read-only.

15. Invalid lifecycle transitions shall be rejected before transaction commit.

16. Historical approvals, committee decisions, execution records, and reconciliation evidence shall never be physically deleted.

---

# 10. Supporting Aggregates

## 10.1 Disposal Case Line Aggregate

### Purpose

The Disposal Case Line represents one inventory item and quantity governed through the disposal/write-off process.

### Owns

- Disposal Line Identifier
- Disposal Case Reference
- Inventory Item Reference
- Item Description Snapshot
- Inventory Classification Snapshot
- Unit of Measure Reference
- Location Reference
- Asset Reference where applicable
- Quantity Proposed
- Quantity Eligible
- Quantity Approved
- Quantity Executed
- Quantity Rejected
- Quantity Pending
- Condition Classification
- Disposal Reason
- Approved Disposal Method
- Valuation Reference
- Line Status

### Invariants

- Every line belongs to exactly one Disposal & Write-off Case.
- Proposed quantity shall be greater than zero.
- Quantities shall never be negative.
- Approved quantity cannot exceed eligible quantity unless a governed authorized exception exists.
- Executed quantity cannot exceed approved quantity.
- Confirmed execution quantity cannot be silently reduced.
- Historical line decisions shall remain traceable.

---

## 10.2 Disposal Assessment Aggregate

### Purpose

Disposal Assessment records the governed evaluation used to determine whether proposed inventory is eligible for disposal/write-off and what treatment is recommended.

### Owns

- Assessment Identifier
- Disposal Case Reference
- Disposal Line Reference
- Assessor Reference
- Assessment Date
- Condition
- Inspection Reference
- Repair Feasibility
- Operational Usefulness
- Eligibility Outcome
- Recommended Disposal Method
- Assessment Remarks
- Supporting Evidence References

### Invariants

- Assessment shall reference an existing case/line.
- Finalized assessment evidence is immutable.
- Revised assessment requires a new version or supplemental assessment.
- Assessment does not itself authorize disposal.

---

## 10.3 Disposal Committee Review Aggregate

### Purpose

The Disposal Committee Review aggregate records formal collective review where institutional policy requires committee governance.

### Owns

- Committee Review Identifier
- Disposal Case Reference
- Committee Reference
- Meeting Reference
- Review Timestamp
- Member References
- Quorum Evidence
- Reviewed Lines
- Recommendation
- Conditions
- Dissenting Remarks
- Supporting Evidence
- Review Status

### Invariants

- Committee review shall occur only where required or explicitly initiated.
- Required quorum shall be satisfied before final recommendation.
- Finalized committee recommendations are immutable.
- A later reconsideration shall create supplemental review evidence rather than overwrite history.

---

## 10.4 Disposal Authorization Aggregate

### Purpose

Disposal Authorization records the institutionally approved authority to execute disposal/write-off.

### Owns

- Authorization Identifier
- Disposal Case Reference
- Authorized Lines
- Authorized Quantities
- Authorized Disposal Methods
- Approval Authority Reference
- Approval Timestamp
- Authorization Conditions
- Validity Period
- Authorization Status

### Invariants

- Authorization requires all mandatory approval prerequisites.
- Authorization shall identify exact applicable scope.
- Authorized quantities cannot be negative.
- Used authorization evidence is immutable.
- Expired or revoked authorization cannot be used for new execution.
- Authorization cannot retrospectively legitimize an already unauthorized disposal.

---

## 10.5 Disposal Execution Aggregate

### Purpose

Disposal Execution records physical implementation of an authorized disposal decision.

### Owns

- Execution Identifier
- Disposal Case Reference
- Authorization Reference
- Execution Date
- Execution Location
- Execution Method
- Executing Officer
- Execution Lines
- Recipient Reference where applicable
- Witness References
- Certificate References
- Handover References
- Execution Remarks
- Supporting Evidence

### Invariants

- Execution requires valid authorization.
- Execution method shall comply with authorization.
- Execution quantity shall not exceed remaining authorized quantity.
- Confirmed execution is immutable.
- Partial execution may be supported.
- Multiple execution records may satisfy one authorization where policy permits.
- Every execution shall preserve responsible actor and timestamp.

---

## 10.6 Disposal Reconciliation Aggregate

### Purpose

Disposal Reconciliation records whether the authorized disposal outcome has been correctly reflected across applicable owning domains.

### Owns

- Reconciliation Identifier
- Disposal Case Reference
- Execution Reference
- Approved Quantity
- Executed Quantity
- Inventory Adjustment Reference
- Asset Retirement Reference
- Financial Processing Reference
- Outstanding Actions
- Reconciliation Status
- Reconciliation Remarks
- Reconciled By
- Reconciliation Timestamp

### Invariants

- Reconciliation shall not invent external-domain confirmations.
- External outcomes shall retain source-domain references.
- Unresolved mandatory actions prevent final closure.
- Final reconciliation evidence is immutable.
# 11. Aggregate Relationships

The principal aggregate relationships within J10 are:

```text
                     Disposal & Write-off Case
                               ¦
          +--------------------+---------------------+
          ¦                    ¦                     ¦
          ?                    ?                     ?
 Disposal Case Line    Disposal Assessment    Committee Review
          ¦                    ¦                     ¦
          +--------------------+---------------------+
                               ¦
                               ?
                    Disposal Authorization
                               ¦
                               ?
                     Disposal Execution
                               ¦
                               ?
                  Disposal Reconciliation
                               ¦
                               ?
                     Completion / Closure
```

Multi-domain relationships are:

```text
Inventory & Warehouse Management (J06)
        ¦
        ¦ Inventory Facts / References
        ?
Disposal, Scrap & Write-Off (J10)
        ¦
        +--------------? Asset Lifecycle Management (J08)
        ¦                Asset-linked Disposal Outcome
        ¦
        +--------------? Finance
        ¦                Approved Financial Consequence Facts
        ¦
        +--------------? Inventory & Warehouse Management (J06)
                         Inventory Adjustment Request
```

Where disposal originates from an inventory audit:

```text
Physical Verification, Audit & Reconciliation (J09)
        ¦
        ¦ Audit Finding /
        ¦ Write-off Recommendation
        ?
Disposal, Scrap & Write-Off (J10)
        ¦
        ?
Governed Disposal / Write-off
```

J10 shall never directly modify persistence owned by J06, J08, J09, Finance, or another bounded context.

---

# 12. Disposal & Write-off Lifecycle

The canonical lifecycle is:

```text
Draft
  ¦
  ?
Submitted
  ¦
  ?
Under Assessment
  ¦
  ?
Assessed
  ¦
  ?
Recommended
  ¦
  +------------? Further Investigation
  ¦                    ¦
  ¦                    ?
  ¦               Re-Assessed
  ¦                    ¦
  +--------------------+
  ¦
  ?
Committee Review
  ¦
  ?
Approval Pending
  ¦
  +------------? Returned
  ¦                  ¦
  ¦                  ?
  ¦              Corrected
  ¦                  ¦
  ¦                  ?
  ¦              Resubmitted
  ¦
  +------------? Rejected
  ¦
  ?
Approved
  ¦
  ?
Authorized
  ¦
  ?
Ready for Execution
  ¦
  +------------? Partially Executed
  ¦                    ¦
  ¦                    ?
  ¦              Awaiting Balance
  ¦                    ¦
  +--------------------+
  ¦
  ?
Executed
  ¦
  ?
Reconciliation Pending
  ¦
  ?
Reconciled
  ¦
  ?
Completed
  ¦
  ?
Closed
  ¦
  ?
Archived
```

Committee Review may be skipped where institutional policy does not require committee governance.

---

## 12.1 Lifecycle State Definitions

| State | Meaning |
|---|---|
| Draft | Disposal/write-off case is being prepared |
| Submitted | Case has been formally submitted |
| Under Assessment | Eligibility and condition assessment is underway |
| Assessed | Required assessment has been completed |
| Recommended | A governed treatment recommendation has been recorded |
| Further Investigation | Additional evidence or investigation is required |
| Committee Review | Case is under formal committee review where required |
| Approval Pending | Required authorization workflow is active |
| Returned | Case has been returned for correction or additional information |
| Rejected | Proposed disposal/write-off has been rejected |
| Approved | Required approval has been obtained |
| Authorized | Executable disposal/write-off authorization exists |
| Ready for Execution | All execution prerequisites have been satisfied |
| Partially Executed | Only part of the approved disposal has been completed |
| Awaiting Balance | Approved quantity remains to be executed or otherwise resolved |
| Executed | Required physical disposal/write-off execution has completed |
| Reconciliation Pending | Multi-domain consequences remain to be reconciled |
| Reconciled | Required inventory/asset/financial references have been reconciled |
| Completed | Business processing is complete |
| Closed | Operational processing has formally ended |
| Archived | Case is retained as immutable historical evidence |

---

## 12.2 Rejection

A rejected case shall preserve:

- Rejection authority
- Rejection timestamp
- Rejection reason
- Applicable workflow reference
- Case state at rejection
- Supporting remarks

Rejection shall not delete the original request or assessment history.

A materially changed disposal proposal should proceed through a new or governed resubmitted case according to institutional policy.

---

## 12.3 Cancellation

Cancellation is a governed terminal outcome available only before irreversible disposal execution has occurred.

Typical cancellation-eligible states may include:

- Draft
- Submitted
- Under Assessment
- Assessed
- Recommended
- Committee Review
- Approval Pending
- Approved
- Authorized
- Ready for Execution

Once physical disposal has been confirmed, cancellation shall not be used to erase the disposal fact.

Post-execution corrections shall be represented through:

- Reconciliation
- Exception handling
- Supplemental evidence
- Corrective inventory processing
- Corrective asset processing
- Corrective financial processing

as applicable.

---

## 12.4 Lifecycle Enforcement

Lifecycle transitions shall be enforced by the Disposal & Write-off Case aggregate.

Every transition shall:

1. Validate the current lifecycle state.
2. Validate actor authorization.
3. Validate tenant and institution context.
4. Validate applicable case invariants.
5. Validate required evidence.
6. Validate external references where required.
7. Reject illegal transitions.
8. Persist the resulting state atomically.
9. Record audit metadata.
10. Publish the applicable domain event after successful commit.

---

# 13. Disposal / Write-off Request Workflow

## Purpose

The Disposal / Write-off Request Workflow initiates the governed process for inventory believed to require terminal disposition or formal write-off.

---

## Workflow

```text
Disposal Requirement Identified
          ¦
          ?
Identify Inventory
          ¦
          ?
Select Disposal / Write-off Reason
          ¦
          ?
Record Proposed Quantity
          ¦
          ?
Reference Location / Custodian
          ¦
          ?
Attach Supporting Evidence
          ¦
          ?
Reference Audit / Asset Finding
          ¦
          ?
Validate Request
          ¦
          ?
Save Draft
```

---

## Request Information

A request should capture:

- Requesting organizational unit
- Requesting actor
- Inventory Item Reference
- Item Description Snapshot
- Inventory location
- Custodian where applicable
- Asset Reference where applicable
- Proposed quantity
- Condition
- Disposal/write-off reason
- Proposed disposal method where known
- Business justification
- Audit Finding Reference where applicable
- Inspection Reference where applicable
- Supporting document references
- Request date
- Priority

---

## Request Rules

- Every request shall belong to one tenant and institution.
- Inventory references shall be validated against J06.
- Proposed quantity shall be greater than zero.
- Asset linkage shall be preserved where applicable.
- An audit-originated request shall preserve the J09 finding reference.
- Creation of a request shall not modify inventory state.
- Creation of a request shall not retire an asset.
- Creation of a request shall not create a financial posting.

---

# 14. Eligibility & Condition Assessment Workflow

## Purpose

The Eligibility & Condition Assessment Workflow determines whether proposed inventory qualifies for disposal/write-off processing and records the supporting institutional evidence.

---

## Workflow

```text
Submitted Case
      ¦
      ?
Review Inventory Reference
      ¦
      ?
Inspect / Review Condition
      ¦
      ?
Review Supporting Evidence
      ¦
      ?
Check Repair / Reuse Feasibility
      ¦
      ?
Check Audit / Asset References
      ¦
      ?
Determine Eligibility
      ¦
  +------------------+
  ¦                  ¦
Eligible         Not Eligible
  ¦                  ¦
  ?                  ?
Recommend         Reject /
Treatment         Return to Use
```

---

## Assessment Factors

Assessment may consider:

- Physical condition
- Operational usability
- Repair feasibility
- Cost-effectiveness of repair
- Obsolescence
- Safety considerations
- Expiry where applicable
- Remaining institutional need
- Availability of alternative use
- Potential internal transfer
- Audit findings
- Asset status
- Historical damage evidence
- Loss/investigation evidence
- Applicable institutional policy

---

## Assessment Rules

- Assessors shall be appropriately authorized.
- Assessment evidence shall identify its source.
- Finalized assessment evidence shall be immutable.
- Changes require supplemental or revised assessment evidence.
- Assessment shall not directly alter J06 inventory.
- Assessment shall not directly retire J08 assets.
- Eligibility shall not automatically constitute disposal authorization.

---

# 15. Disposal Recommendation Workflow

## Purpose

The Disposal Recommendation Workflow determines the proposed treatment for eligible inventory.

---

## Workflow

```text
Assessment Complete
        ¦
        ?
Review Eligibility
        ¦
        ?
Review Condition
        ¦
        ?
Review Institutional Need
        ¦
        ?
Review Disposal Options
        ¦
        ?
Select Recommendation
        ¦
        ?
Record Rationale
        ¦
        ?
Submit for Committee /
Approval Processing
```

---

## Supported Recommendations

A recommendation may include:

- Continue in Service
- Repair
- Reallocate
- Internal Transfer
- Return to Inventory
- Sell
- Auction
- Scrap
- Destroy
- Donate
- Transfer Out
- Recycle
- Write Off
- Further Investigation
- Reject Disposal Proposal

---

## Recommendation Rules

- Recommendation shall reference supporting assessment evidence.
- Recommendation shall identify applicable Disposal Case Lines.
- Different lines within one case may receive different recommendations.
- Recommendation shall not itself authorize execution.
- Any proposed inventory movement back into operational use shall remain subject to the owning domain's rules.

---

# 16. Valuation & Materiality Review Workflow

## Purpose

The Valuation & Materiality Review Workflow obtains the financial or inventory value references required to determine appropriate disposal governance.

J10 does not become the canonical valuation engine.

---

## Workflow

```text
Eligible Disposal Lines
        ¦
        ?
Request Valuation References
        ¦
        ?
Receive Authoritative Values
        ¦
        ?
Determine Materiality
        ¦
        ?
Determine Required Governance
        ¦
        +--------? Standard Approval
        ¦
        +--------? Committee Review
        ¦
        +--------? Higher Authority Review
```

---

## Possible Value References

J10 may consume:

- Inventory cost reference
- Book value reference
- Net book value reference
- Estimated residual value
- Scrap value estimate
- Market value estimate
- Auction reserve reference
- Replacement-value reference

The authoritative source depends on the applicable inventory, asset, and finance architecture.

---

## Valuation Rules

- J10 shall preserve the source of valuation information.
- J10 shall not overwrite authoritative valuation records.
- Materiality policy may determine approval level.
- Changes in authoritative valuation before approval may require re-evaluation.
- Transaction-time value snapshots may be preserved for historical decision evidence.

---

# 17. Disposal Committee Review Workflow

## Purpose

The Disposal Committee Review Workflow provides formal collective governance where institutional policy requires committee consideration before disposal/write-off authorization.

---

## Workflow

```text
Case Recommended
       ¦
       ?
Determine Committee Requirement
       ¦
   +---------------+
   ¦               ¦
Not Required    Required
   ¦               ¦
   ¦               ?
   ¦        Schedule / Reference Review
   ¦               ¦
   ¦               ?
   ¦          Validate Quorum
   ¦               ¦
   ¦               ?
   ¦          Review Evidence
   ¦               ¦
   ¦               ?
   ¦        Record Recommendation
   ¦               ¦
   +---------------+
           ¦
           ?
     Approval Processing
```

---

## Committee Review Information

The review may record:

- Committee Reference
- Meeting Reference
- Meeting Date
- Member References
- Chairperson Reference
- Quorum Evidence
- Disposal Case Reference
- Reviewed Lines
- Condition evidence
- Valuation references
- Audit findings
- Asset references
- Recommendation
- Conditions
- Dissenting remarks
- Supporting documents

---

## Committee Rules

- Committee requirements shall be policy-driven.
- Required quorum shall be satisfied.
- Committee recommendation shall not be silently changed after finalization.
- A revised committee decision shall create additional evidence.
- Committee review shall not itself modify inventory or accounting state.
- Committee members and approving authority may be subject to segregation-of-duties policy.

---

# 18. Approval & Authorization Workflow

## Purpose

The Approval & Authorization Workflow obtains formal institutional authority before disposal/write-off execution.

Workflow routing remains owned by the enterprise Workflow Engine.

---

## Workflow

```text
Recommended Case
       ¦
       ?
Validate Approval Preconditions
       ¦
       ?
Submit to Workflow Engine
       ¦
       +--------? Returned
       ¦              ¦
       ¦              ?
       ¦          Corrected
       ¦              ¦
       ¦              ?
       ¦         Resubmitted
       ¦
       +--------? Rejected
       ¦
       ?
Approved
       ¦
       ?
Create Disposal Authorization
       ¦
       ?
Ready for Execution
```

---

## Approval Preconditions

Approval may require:

- Completed assessment
- Eligibility confirmation
- Disposal recommendation
- Valuation references
- Committee recommendation where required
- Supporting evidence
- Audit reference where applicable
- Asset review where applicable
- Required investigation completion
- Required authority level

---

## Approval Rules

- J10 records approval outcome but does not own workflow routing.
- Approval authority may depend on materiality, category, quantity, disposal method, asset class, or institutional policy.
- Approval history shall remain immutable.
- Rejected cases cannot proceed to execution.
- Returned cases may be corrected and resubmitted.
- Authorization shall identify exact approved scope.
- Approval of one line shall not imply approval of every line.
- Execution shall not exceed authorization.

---

# 19. Physical Disposal Execution Workflow

## Purpose

The Physical Disposal Execution Workflow records actual removal of approved inventory from institutional custody through the authorized disposal method.

---

## Workflow

```text
Authorized Disposal
        ¦
        ?
Prepare Execution
        ¦
        ?
Verify Inventory / Quantity
        ¦
        ?
Verify Authorization
        ¦
        ?
Verify Disposal Method
        ¦
        ?
Record Recipient / Witnesses
        ¦
        ?
Execute Disposal
        ¦
        ?
Capture Evidence
        ¦
        ?
Confirm Execution
        ¦
        ?
Publish DisposalExecuted
```

---

## Execution Information

Execution may capture:

- Execution Identifier
- Case Reference
- Authorization Reference
- Execution Date
- Execution Location
- Inventory Item Reference
- Approved Quantity
- Executed Quantity
- Disposal Method
- Executing Officer
- Recipient Reference
- Witness References
- Package/lot references where applicable
- Certificate Reference
- Handover Reference
- Supporting document references
- Execution remarks

---

## Execution Rules

- Only authorized disposal may be executed.
- Authorization shall remain valid at execution time.
- Inventory identity shall be verified before execution.
- Executed quantity shall not exceed remaining authorized quantity.
- Confirmed execution records shall be immutable.
- Partial execution may be supported.
- Execution evidence shall preserve actor and timestamp.
- Disposal method shall match authorization.
- Physical execution shall generate the appropriate domain event.
- J10 shall not directly reduce J06 stock quantities.

---

# 20. Destruction Workflow

## Purpose

The Destruction Workflow governs disposal where inventory must be physically destroyed rather than transferred to another recipient.

---

## Workflow

```text
Authorized Destruction
        ¦
        ?
Verify Items
        ¦
        ?
Verify Quantities
        ¦
        ?
Verify Required Witnesses
        ¦
        ?
Perform Destruction
        ¦
        ?
Record Destruction Evidence
        ¦
        ?
Generate / Reference Certificate
        ¦
        ?
Confirm Destruction
```

---

## Destruction Evidence

Evidence may include:

- Destruction date
- Destruction location
- Destruction method classification
- Executing authority
- Witnesses
- Item references
- Quantities
- Certificate reference
- Supporting evidence references
- Remarks

J10 shall remain technology-neutral regarding how supporting evidence is physically stored.

---

## Destruction Rules

- Destruction requires explicit authorization.
- Required witness policy shall be enforced.
- Destruction quantity shall not exceed authorization.
- Confirmed destruction is irreversible within J10.
- Corrections shall be represented through supplemental evidence.
- Environmental, hazardous-material, or specialized regulatory handling shall remain subject to applicable external institutional policies and specialized capabilities.

---

# 21. Scrap Disposal Workflow

## Purpose

The Scrap Disposal Workflow governs inventory authorized for disposal as scrap.

---

## Workflow

```text
Scrap Authorization
       ¦
       ?
Identify Scrap Inventory
       ¦
       ?
Verify Quantity / Weight
       ¦
       ?
Identify Approved Recipient
       ¦
       ?
Record Handover
       ¦
       ?
Record Scrap Outcome
       ¦
       ?
Capture Evidence
       ¦
       ?
Confirm Execution
```

---

## Scrap Information

J10 may record:

- Scrap classification
- Inventory Item Reference
- Quantity
- Weight where applicable
- Recipient Reference
- Handover date
- Handover officer
- Witnesses
- Scrap value reference
- Receipt/reference number
- Supporting evidence

---

## Scrap Rules

- Scrap execution requires approved disposal authorization.
- Recipient identity shall be recorded where applicable.
- J10 shall not become the canonical vendor/recipient master.
- Sale proceeds or accounting consequences remain owned by Finance.
- Scrap execution evidence shall be immutable after confirmation.

---

# 22. Sale & Auction Disposal Workflow

## Purpose

The Sale & Auction Disposal Workflow governs disposal where institution-owned inventory is authorized to leave custody through sale or auction.

J10 governs the disposal outcome but does not become a procurement, auction-platform, receivables, or financial-settlement system.

---

## Workflow

```text
Sale / Auction Authorization
          ¦
          ?
Reference Approved Disposal Process
          ¦
          ?
Record Successful Outcome
          ¦
          ?
Record Recipient / Buyer Reference
          ¦
          ?
Record Approved Quantity
          ¦
          ?
Record Sale / Auction Reference
          ¦
          ?
Confirm Physical Handover
          ¦
          ?
Capture Evidence
          ¦
          ?
Publish Disposal Execution Facts
```

---

## Sale / Auction References

J10 may preserve:

- Sale Reference
- Auction Reference
- Lot Reference
- Buyer/Recipient Reference
- Approved quantity
- Handover quantity
- Sale-value reference
- Financial transaction reference
- Receipt reference
- Handover evidence

---

## Rules

- J10 shall not independently conduct financial settlement.
- J10 shall not post sale proceeds.
- Buyer master data shall remain externally owned where applicable.
- Physical handover requires valid disposal authorization.
- Financial confirmation may be required before handover according to institutional policy.
- Sale/auction outcome evidence shall remain auditable.

---

# 23. Donation & External Transfer-out Workflow

## Purpose

The Donation & External Transfer-out Workflow governs authorized transfer of institutional inventory to an external organization or other approved recipient without normal internal stock-transfer semantics.

---

## Workflow

```text
Donation / Transfer-out Approval
            ¦
            ?
Identify Recipient
            ¦
            ?
Verify Authorized Inventory
            ¦
            ?
Prepare Handover
            ¦
            ?
Record Recipient Acknowledgement
            ¦
            ?
Confirm Handover
            ¦
            ?
Capture Evidence
            ¦
            ?
Confirm Disposal Execution
```

---

## Rules

- Donation requires explicit authorization.
- Recipient shall be identifiable.
- Handover quantity shall not exceed approved quantity.
- Recipient acknowledgement shall be retained where required.
- J07 shall be used for internal institutional stock transfers rather than incorrectly modeling normal internal movement as disposal.
- External handover evidence shall remain immutable after confirmation.

---

# 24. Inventory Loss & Write-off Workflow

## Purpose

The Inventory Loss & Write-off Workflow governs cases where inventory cannot be physically disposed because it is already missing, lost, destroyed, or otherwise unavailable, but formal institutional write-off authorization is required.

---

## Workflow

```text
Loss / Missing Inventory Identified
           ¦
           ?
Reference Audit / Incident Evidence
           ¦
           ?
Validate Investigation Requirement
           ¦
           ?
Complete Required Investigation
           ¦
           ?
Assess Write-off Eligibility
           ¦
           ?
Committee / Authority Review
           ¦
           ?
Approve Write-off
           ¦
           ?
Create Write-off Authorization
           ¦
           ?
Request J06 Inventory Adjustment
           ¦
           ?
Reconcile Outcome
```

---

## Write-off Reasons

Write-off may include:

- Confirmed loss
- Missing inventory
- Irrecoverable shortage
- Destroyed inventory
- Theft-related loss after governed investigation
- Damage with no recoverable physical stock
- Audit-confirmed unrecoverable variance
- Other institution-approved reason

---

## Write-off Rules

- Write-off shall require sufficient evidence.
- Missing inventory shall not be automatically written off merely because a count variance exists.
- J09 audit findings may initiate or support the case but do not replace J10 authorization.
- Required investigation shall complete before write-off where policy mandates it.
- J10 authorizes the business write-off outcome.
- J06 executes the authoritative inventory adjustment.
- Finance owns financial recognition.
- Historical loss evidence shall remain immutable.

---

# 25. Inventory Adjustment Integration Workflow

## Purpose

The Inventory Adjustment Integration Workflow communicates an approved disposal/write-off outcome to J06 for canonical inventory processing.

---

## Workflow

```text
Disposal / Write-off Executed
          ¦
          ?
Prepare Adjustment Request
          ¦
          ?
Validate Execution / Authorization
          ¦
          ?
Send Request to J06
          ¦
          ?
J06 Validates Inventory State
          ¦
     +-------------+
     ¦             ¦
 Accepted       Rejected
     ¦             ¦
     ?             ?
Inventory       Exception /
Adjusted        Reconciliation
     ¦
     ?
Return Adjustment Reference
     ¦
     ?
Record in J10
```

---

## Integration Rules

- J10 shall never directly update J06 storage.
- Adjustment requests shall identify the disposal/write-off case.
- Adjustment requests shall identify applicable inventory items and quantities.
- Requests shall be idempotent.
- J06 shall return authoritative processing status.
- J10 shall preserve the returned adjustment reference.
- J06 rejection shall not erase the approved disposal fact.
- Rejected or uncertain adjustment outcomes shall enter reconciliation/exception processing.

---

# 26. Asset Retirement Integration Workflow

## Purpose

The Asset Retirement Integration Workflow communicates applicable disposal facts to J08 when disposed inventory is linked to a canonical institutional asset.

---

## Workflow

```text
Asset-linked Disposal Executed
          ¦
          ?
Identify Asset References
          ¦
          ?
Publish Disposal Outcome
          ¦
          ?
Asset Lifecycle Management (J08)
          ¦
          ?
Validate Asset State
          ¦
          ?
Process Asset Retirement
          ¦
          ?
Return Asset Outcome Reference
          ¦
          ?
Record Reconciliation Reference
```

---

## Rules

- J10 shall not directly change asset lifecycle state.
- J08 determines valid asset retirement processing.
- Asset retirement failure shall not erase physical disposal evidence.
- Exceptions shall be reconciled.
- J10 shall preserve the applicable J08 reference.
- Asset and inventory consequences may complete independently and shall be reconciled before final closure where required.

---

# 27. Financial Consequence Integration

## Purpose

Disposal/write-off may create financial consequences that require processing by Finance.

J10 provides authoritative disposal/write-off business facts but does not perform accounting.

---

## Possible Financial Consequences

Depending on institutional policy and the disposed item, consequences may include:

- Inventory write-off
- Asset derecognition
- Loss recognition
- Gain/loss on disposal
- Scrap proceeds
- Sale proceeds
- Auction proceeds
- Receivable creation
- Tax consequences where applicable
- Other financial adjustments

---

## Integration Principle

```text
J10 Disposal / Write-off Fact
          ¦
          ?
Finance Integration Contract
          ¦
          ?
Finance Domain
          ¦
          ?
Accounting Decision / Posting
          ¦
          ?
Financial Reference
          ¦
          ?
J10 Reconciliation
```

Finance remains authoritative for:

- Accounting policy
- Journal construction
- Ledger posting
- Financial period
- Account determination
- Monetary recognition
- Financial balances

---

# 28. Disposal Reconciliation Workflow

## Purpose

The Disposal Reconciliation Workflow confirms that the approved institutional disposal/write-off outcome has been correctly reflected across all mandatory participating domains.

---

## Workflow

```text
Execution Complete
       ¦
       ?
Check Approved vs Executed Quantity
       ¦
       ?
Check J06 Adjustment
       ¦
       ?
Check J08 Outcome if Applicable
       ¦
       ?
Check Finance Reference if Required
       ¦
       ?
Check Evidence Completeness
       ¦
   +-----------------+
   ¦                 ¦
Complete         Exception
   ¦                 ¦
   ?                 ?
Reconciled      Resolution Required
```

---

## Reconciliation Dimensions

J10 may reconcile:

- Quantity proposed
- Quantity eligible
- Quantity approved
- Quantity physically executed
- Quantity written off
- J06 adjusted quantity
- J08 asset outcome
- Finance processing reference
- Recipient handover
- Required certificates
- Required witness evidence
- Outstanding exceptions

---

## Reconciliation Rules

- Approved and executed quantities shall be deterministically reconcilable.
- J06 outcomes shall be consumed as authoritative inventory facts.
- J08 outcomes shall be consumed as authoritative asset facts.
- Finance outcomes shall be consumed as authoritative financial facts.
- Missing mandatory outcomes shall prevent final closure where policy requires them.
- Reconciliation shall never modify external-domain records directly.

---

# 29. Disposal Exception Workflow

## Purpose

The Disposal Exception Workflow governs abnormal conditions preventing normal disposal/write-off completion.

---

## Exception Examples

- Inventory not found at execution
- Quantity mismatch
- Wrong item presented
- Authorization expired
- Authorization revoked
- Disposal method mismatch
- Recipient unavailable
- Required witness unavailable
- Certificate missing
- Execution quantity exceeds approval
- J06 adjustment rejected
- Asset retirement rejected
- Financial processing rejected
- Duplicate execution attempt
- Evidence incomplete
- Multi-domain reference mismatch

---

## Workflow

```text
Exception Detected
       ¦
       ?
Classify Exception
       ¦
       ?
Preserve Current State
       ¦
       ?
Record Evidence
       ¦
       ?
Determine Recoverability
       ¦
   +----------------+
   ¦                ¦
Recoverable     Non-Recoverable
   ¦                ¦
   ?                ?
Correct / Retry   Escalate /
   ¦             Governed Resolution
   ?                ¦
Resume              ?
Processing      Record Outcome
```

---

## Exception Rules

- Exceptions shall never be silently ignored.
- Failed execution shall not be represented as successful disposal.
- External-domain failures shall preserve their source references.
- Retry shall be permitted only where the operation is safely retryable.
- Confirmed physical disposal shall never be repeated merely because downstream integration failed.
- Manual resolution shall preserve actor, reason, timestamp, and resulting state.

---

# 30. Disposal Cancellation Workflow

## Purpose

The Disposal Cancellation Workflow provides controlled termination of a case before irreversible disposal/write-off execution.

---

## Workflow

```text
Eligible Case
     ¦
     ?
Request Cancellation
     ¦
     ?
Validate Lifecycle
     ¦
     ?
Validate Authorization
     ¦
     ?
Check Execution History
     ¦
  +---------------+
  ¦               ¦
None          Execution Exists
  ¦               ¦
  ?               ?
Cancel         Reject Normal
Case           Cancellation
```

---

## Cancellation Rules

- Cancellation requires authorization.
- Cases with confirmed physical execution cannot use cancellation to erase execution history.
- Unused authorization may be revoked or cancelled according to policy.
- Cancellation reason shall be mandatory.
- Cancellation evidence shall be immutable.
- Cancellation shall not automatically modify J06, J08, or Finance records.

---

# 31. Completion, Closure & Archival Workflow

## Purpose

Completion confirms that required disposal/write-off business processing has concluded.

Closure formally ends operational processing.

Archival preserves the case as immutable institutional evidence.

---

## Completion Preconditions

A case may be completed only when:

- Required disposal/write-off execution is complete.
- Approved and executed quantities are reconciled.
- Mandatory disposal evidence exists.
- Required inventory adjustment has reached an acceptable outcome.
- Required asset retirement references exist where applicable.
- Required financial processing references exist where applicable.
- Mandatory exceptions are resolved.
- Required committee and approval evidence is preserved.
- Required custody/handover evidence is preserved.

---

## Workflow

```text
Reconciliation Complete
        ¦
        ?
Validate Completion Preconditions
        ¦
        ?
Complete Case
        ¦
        ?
Publish DisposalCaseCompleted
        ¦
        ?
Close Case
        ¦
        ?
Archive
```

---

## Completion Rules

- Completion is a governed lifecycle transition.
- Completed cases cannot accept new disposal execution.
- Closure occurs only after required operational activities finish.
- Closed cases are read-only.
- Archived cases cannot return to an operational state.
- Historical evidence shall never be physically deleted through normal business operations.

---

# 32. Business Rules

## 32.1 General Rules

- Every Disposal & Write-off Case belongs to exactly one tenant and institution.
- Every submitted case shall contain at least one valid Disposal Case Line.
- Case identity is immutable.
- Case history is immutable.
- Lifecycle transitions shall follow the canonical state model.
- Illegal transitions shall be rejected.
- Disposal shall never be executed without required authorization.

---

## 32.2 Inventory Boundary Rules

- J10 shall never maintain a competing canonical inventory balance.
- Inventory references are validated through J06.
- Inventory adjustments are executed by J06.
- J10 stores J06 adjustment references.
- J10 shall never directly modify J06 persistence.
- Physical disposal and inventory adjustment are separate business facts.
- J06 integration failure shall not erase physical disposal evidence.

---

## 32.3 Quantity Rules

For each Disposal Case Line:

- Proposed Quantity > 0.
- Eligible Quantity = 0.
- Approved Quantity = 0.
- Executed Quantity = 0.
- Rejected Quantity = 0.
- Pending Quantity = 0.
- Executed Quantity shall not exceed authorized quantity.
- Quantities shall remain reconcilable throughout the lifecycle.

---

## 32.4 Approval Rules

- Approval shall occur only after required prerequisites.
- Approval authority shall satisfy institutional policy.
- Approval history is immutable.
- Returned cases may be corrected and resubmitted.
- Rejected cases cannot proceed to execution.
- Authorization shall define exact executable scope.

---

## 32.5 Execution Rules

- Execution requires valid authorization.
- Execution shall use the authorized disposal method.
- Execution quantity shall remain within approved limits.
- Confirmed execution is immutable.
- Partial execution may be supported.
- Every execution shall identify responsible actors.
- Required witnesses shall be recorded.
- Required evidence shall be retained.

---

## 32.6 Write-off Rules

- Write-off shall require sufficient supporting evidence.
- Audit variance alone does not automatically constitute write-off authorization.
- Required investigation shall be completed where applicable.
- J10 owns write-off authorization.
- J06 owns inventory adjustment.
- Finance owns accounting consequences.

---

## 32.7 Asset Rules

- Asset-linked inventory shall preserve the asset reference.
- J10 shall not retire the asset directly.
- J08 owns canonical asset retirement.
- Asset-processing exceptions shall remain reconcilable.

---

## 32.8 Disposal Evidence Rules

- Finalized disposal evidence shall be immutable.
- Evidence shall preserve provenance.
- Evidence correction shall use supplemental records rather than destructive replacement.
- Required certificates shall remain associated with the applicable execution.
- Evidence access shall follow security and retention policy.

---

## 32.9 Segregation-of-Duties Rules

Institutional policy may prohibit the same actor from performing conflicting responsibilities such as:

- Requesting and final approval
- Assessment and final approval
- Approval and disposal execution
- Execution and independent witness confirmation
- Disposal execution and reconciliation approval

Segregation-of-duties rules shall be policy-driven and enforced through authorization/workflow integration.

---

## 32.10 Closure Rules

- Mandatory unresolved exceptions prevent closure.
- Required reconciliation shall complete before closure.
- Completed cases cannot accept new execution.
- Closed cases are read-only.
- Archived cases are permanently non-operational.

---

# 33. Commands

Commands express business intent and modify J10 aggregate state.

Commands shall execute only after authentication, authorization, validation, and aggregate invariant enforcement.

---

## 33.1 Disposal Case Commands

| Command | Purpose |
|---|---|
| CreateDisposalCase | Create a disposal/write-off case |
| UpdateDisposalCase | Modify an eligible case |
| SubmitDisposalCase | Submit case for processing |
| CancelDisposalCase | Cancel an eligible case |
| CompleteDisposalCase | Complete reconciled case |
| CloseDisposalCase | Close completed case |
| ArchiveDisposalCase | Archive closed case |

---

## 33.2 Disposal Line Commands

| Command | Purpose |
|---|---|
| AddDisposalCaseLine | Add inventory item to case |
| UpdateDisposalCaseLine | Modify eligible case line |
| RemoveDisposalCaseLine | Remove eligible case line |
| UpdateProposedDisposalQuantity | Change proposed quantity before protected states |
| RecordDisposalLineCondition | Record item condition |

---

## 33.3 Assessment Commands

| Command | Purpose |
|---|---|
| StartDisposalAssessment | Begin eligibility assessment |
| RecordConditionAssessment | Record condition evidence |
| RecordDisposalEligibility | Record eligibility decision |
| RecordDisposalRecommendation | Record proposed treatment |
| RequestFurtherInvestigation | Require additional investigation |
| FinalizeDisposalAssessment | Finalize assessment |

---

## 33.4 Committee Commands

| Command | Purpose |
|---|---|
| CreateDisposalCommitteeReview | Create committee review record |
| RecordCommitteeAttendance | Record participating members |
| RecordCommitteeQuorum | Record quorum evidence |
| RecordCommitteeRecommendation | Record committee recommendation |
| FinalizeCommitteeReview | Finalize committee evidence |

---

## 33.5 Approval & Authorization Commands

| Command | Purpose |
|---|---|
| SubmitDisposalForApproval | Initiate approval workflow |
| RecordDisposalApproval | Record approved workflow outcome |
| RecordDisposalReturn | Record returned workflow outcome |
| RecordDisposalRejection | Record rejection |
| CreateDisposalAuthorization | Create approved execution authorization |
| RevokeDisposalAuthorization | Revoke unused authorization where permitted |

---

## 33.6 Execution Commands

| Command | Purpose |
|---|---|
| CreateDisposalExecution | Create execution operation |
| AddDisposalExecutionLine | Add authorized line to execution |
| RecordDisposalRecipient | Record recipient reference |
| RecordDisposalWitness | Record witness evidence |
| RecordDisposalCertificate | Record certificate reference |
| ConfirmDisposalExecution | Confirm physical disposal |
| ConfirmDestruction | Confirm authorized destruction |
| ConfirmScrapHandover | Confirm scrap handover |
| ConfirmDonationHandover | Confirm donation handover |
| ConfirmExternalTransferOut | Confirm external transfer-out |

---

## 33.7 Write-off Commands

| Command | Purpose |
|---|---|
| CreateWriteOffRequest | Create write-off request |
| RecordLossEvidence | Record loss/missing evidence |
| RecordInvestigationReference | Record governed investigation reference |
| ApproveInventoryWriteOff | Record approved write-off outcome |
| RequestInventoryWriteOffAdjustment | Request J06 adjustment |

---

## 33.8 Reconciliation Commands

| Command | Purpose |
|---|---|
| RecordInventoryAdjustmentReference | Record J06 processing reference |
| RecordAssetRetirementReference | Record J08 processing reference |
| RecordFinancialProcessingReference | Record Finance reference |
| RecordDisposalReconciliationException | Record reconciliation exception |
| ResolveDisposalReconciliationException | Resolve reconciliation exception |
| CompleteDisposalReconciliation | Finalize reconciliation |

---

## 33.9 Command Idempotency

Commands that may be retried because of network, messaging, workflow, or integration failure shall support idempotent execution.

Idempotency-sensitive commands include:

- SubmitDisposalCase
- RecordDisposalApproval
- CreateDisposalAuthorization
- ConfirmDisposalExecution
- ConfirmDestruction
- ConfirmScrapHandover
- ConfirmDonationHandover
- ApproveInventoryWriteOff
- RequestInventoryWriteOffAdjustment
- RecordInventoryAdjustmentReference
- CompleteDisposalReconciliation
- CompleteDisposalCase
- CloseDisposalCase

Command processing should preserve:

- Command Identifier
- Idempotency Key
- Aggregate Identifier
- Correlation Identifier
- Causation Identifier
- Expected Aggregate Version
- Actor Context
- Tenant Context
- Institution Context

Duplicate execution shall not create duplicate disposal executions, certificates, write-offs, adjustment requests, or completion events.

---

# 34. Domain Events

Domain events represent immutable business facts produced by J10.

Events shall be published only after the corresponding state transition has successfully committed.

---

## 34.1 Disposal Case Events

- DisposalCaseCreated
- DisposalCaseUpdated
- DisposalCaseSubmitted
- DisposalCaseReturned
- DisposalCaseRejected
- DisposalCaseCancelled
- DisposalCaseCompleted
- DisposalCaseClosed
- DisposalCaseArchived

---

## 34.2 Assessment Events

- DisposalAssessmentStarted
- ConditionAssessmentRecorded
- DisposalEligibilityConfirmed
- DisposalEligibilityRejected
- DisposalRecommendationRecorded
- FurtherInvestigationRequested
- DisposalAssessmentFinalized

---

## 34.3 Committee Events

- DisposalCommitteeReviewCreated
- DisposalCommitteeQuorumConfirmed
- DisposalCommitteeRecommendationRecorded
- DisposalCommitteeReviewFinalized

---

## 34.4 Approval Events

- DisposalApprovalRequested
- DisposalApproved
- DisposalApprovalReturned
- DisposalRejected
- DisposalAuthorizationCreated
- DisposalAuthorizationRevoked

---

## 34.5 Execution Events

- DisposalExecutionCreated
- DisposalExecutionConfirmed
- DisposalPartiallyExecuted
- DisposalFullyExecuted
- InventoryDestroyed
- ScrapHandoverConfirmed
- DonationHandoverConfirmed
- ExternalTransferOutConfirmed
- DisposalCertificateRecorded

---

## 34.6 Write-off Events

- InventoryWriteOffRequested
- InventoryLossEvidenceRecorded
- InventoryWriteOffApproved
- InventoryWriteOffRejected
- InventoryAdjustmentRequested

---

## 34.7 Reconciliation Events

- InventoryAdjustmentReferenceRecorded
- AssetRetirementReferenceRecorded
- FinancialProcessingReferenceRecorded
- DisposalReconciliationExceptionRecorded
- DisposalReconciliationExceptionResolved
- DisposalReconciled

---

# 35. Event Envelope

Every integration-relevant J10 event shall carry a standard enterprise event envelope.

At minimum:

```text
Event ID
Event Type
Event Version
Aggregate ID
Aggregate Type
Tenant ID
Institution ID
Occurred At
Published At
Correlation ID
Causation ID
Actor / System Context
Source Domain
Schema Version
```

Where applicable, events may additionally include:

- Disposal Case Number
- Disposal Line Identifier
- Inventory Item Reference
- Location Reference
- Asset Reference
- Disposal Method
- Quantity
- Authorization Reference
- Execution Reference
- Adjustment Reference

Sensitive evidence shall not be indiscriminately embedded within event payloads.

Events should reference protected evidence where appropriate.

---

# 36. Event Delivery & Reliability

J10 shall use reliable event publication patterns so that aggregate commit and integration-event publication cannot silently diverge.

The logical sequence is:

```text
Command
   ¦
   ?
Validate Aggregate
   ¦
   ?
Commit State
   ¦
   ?
Persist Event for Publication
   ¦
   ?
Publish Event
   ¦
   ?
Consumer Processing
```

The architecture may use an enterprise-approved transactional outbox or equivalent reliability mechanism.

Implementation technology remains a Level 3 concern.

---

## 36.1 Consumer Idempotency

Consumers shall treat J10 integration events as potentially redeliverable.

Consumer processing should use:

- Event ID
- Aggregate ID
- Event Version
- Correlation ID
- Business Reference

Duplicate event delivery shall not cause duplicate:

- Inventory adjustments
- Asset retirement
- Financial posting
- Notifications
- Audit evidence
- Dashboard facts

---

## 36.2 Event Ordering

Events affecting the same Disposal & Write-off Case shall preserve sufficient aggregate version information for consumers to detect:

- Duplicate delivery
- Out-of-order delivery
- Missing expected versions

Global ordering across all disposal cases is not required.

---

# 37. Queries & Read Models

CQRS separates state-changing commands from read-optimized query models.

J10 shall expose authorized read models without transferring canonical ownership.

---

## 37.1 Disposal Case Queries

Representative queries include:

- GetDisposalCaseById
- GetDisposalCaseByNumber
- SearchDisposalCases
- GetDisposalCaseLines
- GetDisposalCaseLifecycle
- GetDisposalCaseHistory
- GetDisposalCaseEvidence

---

## 37.2 Operational Queries

Representative operational queries include:

- GetPendingAssessments
- GetPendingCommitteeReviews
- GetPendingApprovals
- GetApprovedAwaitingExecution
- GetPartiallyExecutedCases
- GetPendingInventoryAdjustments
- GetPendingAssetRetirements
- GetPendingFinancialProcessing
- GetReconciliationExceptions
- GetCasesReadyForClosure

---

## 37.3 Audit Queries

Authorized users may query:

- Disposal authorization history
- Committee decision history
- Execution history
- Witness history
- Certificate references
- Inventory adjustment references
- Asset retirement references
- Financial references
- Reconciliation history
- Cancellation history

Read models shall not permit modification of immutable evidence.

---

# 38. Multi-Domain Integration Contracts

J10 integrates with other SutraERP bounded contexts only through governed contracts.

---

## 38.1 J06 — Inventory & Warehouse Management

J10 consumes:

- Inventory item references
- Inventory location references
- Inventory status where required
- Inventory quantity validation
- Inventory adjustment outcome

J10 publishes/provides:

- Approved disposal facts
- Physical disposal facts
- Approved write-off facts
- Inventory adjustment requests

J06 remains authoritative for inventory state.

---

## 38.2 J08 — Asset Lifecycle Management

J10 consumes:

- Asset references
- Asset linkage
- Asset status where required
- Asset retirement outcome

J10 provides:

- Approved asset-linked disposal fact
- Physical disposal confirmation
- Disposal method
- Execution evidence reference

J08 remains authoritative for asset lifecycle state.

---

## 38.3 J09 — Physical Verification, Audit & Reconciliation

J10 may consume:

- Audit Finding Reference
- Audit Variance Reference
- Damage Finding
- Missing Inventory Finding
- Write-off Recommendation

J10 shall preserve J09 provenance.

J10 shall not modify J09 audit history.

---

## 38.4 Finance

J10 may provide:

- Approved write-off fact
- Disposal execution fact
- Sale/auction outcome reference
- Scrap outcome reference
- Asset-linked disposal fact
- Financial consequence reference data

Finance returns applicable financial processing references.

Finance remains authoritative for accounting.

---

## 38.5 Workflow Engine

J10 provides:

- Case identity
- Disposal type
- Materiality
- Approval context
- Committee outcome
- Requested action

Workflow Engine provides:

- Workflow Instance ID
- Approval outcome
- Return outcome
- Rejection outcome
- Approval actor
- Decision timestamp

J10 owns the resulting disposal lifecycle state.

Workflow Engine owns routing/orchestration.

---

## 38.6 Notification Infrastructure

J10 publishes notification-triggering facts.

The enterprise Notification Platform owns delivery through configured channels.

J10 shall not independently implement email, SMS, push, or other notification infrastructure.

---

## 38.7 Audit Infrastructure

J10 generates domain audit evidence.

Enterprise audit infrastructure owns canonical audit persistence, retention governance, and enterprise-wide audit access.

---

# 39. Integration Failure Handling

Multi-domain failures shall not corrupt committed J10 business facts.

Example:

```text
Physical Disposal Confirmed
          ¦
          ?
J10 Commit Successful
          ¦
          ?
Inventory Adjustment Request
          ¦
          ?
J06 Temporarily Unavailable
          ¦
          ?
Retry / Exception Processing
          ¦
          ?
Reconciliation Pending
```

J10 shall not reverse confirmed physical disposal merely because J06 is temporarily unavailable.

Similarly, a Finance or J08 failure shall become a reconciliation concern rather than rewriting historical disposal evidence.

---

# 40. Concurrency & Optimistic Locking

Concurrent updates to the same disposal aggregate shall be controlled through aggregate versioning or equivalent concurrency mechanisms.

Representative protected operations include:

- Case modification
- Assessment finalization
- Committee recommendation
- Approval recording
- Authorization creation
- Disposal execution
- Write-off approval
- Reconciliation completion
- Case closure

A command operating against a stale aggregate version shall fail safely and require re-evaluation.

Concurrency conflicts shall never silently overwrite:

- Approval state
- Authorization scope
- Executed quantities
- Disposal evidence
- Reconciliation state

---

# 41. Security & Authorization Architecture

## 41.1 Security Principle

Disposal and write-off represent high-risk institutional actions because they may permanently remove inventory from custody and create financial consequences.

Every material action shall therefore require explicit authentication and authorization.

---

## 41.2 Representative Authorization Capabilities

Permissions may include:

- Create Disposal Case
- Modify Disposal Case
- Submit Disposal Case
- Assess Disposal Eligibility
- Record Condition Assessment
- Record Disposal Recommendation
- Participate in Disposal Committee
- Approve Disposal
- Authorize Write-off
- Execute Disposal
- Witness Disposal
- Record Disposal Evidence
- Request Inventory Adjustment
- Reconcile Disposal
- Close Disposal Case
- View Sensitive Disposal Evidence

Authorization shall remain policy-driven.

---

## 41.3 Contextual Authorization

Authorization decisions may consider:

- Tenant
- Institution
- Campus
- Department
- Warehouse
- Inventory category
- Asset classification
- Disposal method
- Quantity
- Materiality
- Case lifecycle state
- Actor role
- Committee membership
- Approval authority
- Segregation-of-duties constraints

Successful authentication alone shall never imply permission to dispose of inventory.

---

# 42. Segregation of Duties

J10 shall support institutional segregation-of-duties policy.

A single actor should not automatically possess unrestricted ability to:

```text
Create Disposal Request
        +
Assess Eligibility
        +
Approve Disposal
        +
Execute Disposal
        +
Confirm Witness Evidence
        +
Reconcile Outcome
```

Policy may enforce separation between:

- Requester and approver
- Assessor and approver
- Committee member and final approving authority
- Approver and execution officer
- Execution officer and witness
- Execution officer and reconciliation officer

Emergency exceptions, if permitted, shall require explicit authorization and enhanced audit evidence.

---

# 43. Tenant & Institution Isolation

Every J10 aggregate, command, query, event, and integration interaction shall preserve:

- Tenant ID
- Institution ID

Cross-tenant access shall be prohibited.

Cross-institutional disposal processing shall not be inferred from shared user identity, shared warehouse configuration, or common inventory classification.

Any shared-service processing shall preserve institutional ownership and authorization boundaries.

---

# 44. Audit & Evidence Architecture

## 44.1 Audit Principle

Every material disposal/write-off action shall produce sufficient evidence to reconstruct:

- What inventory was proposed
- Why disposal/write-off was requested
- Who assessed it
- What evidence was considered
- What recommendation was made
- Who approved it
- What was authorized
- What was physically executed
- Who witnessed execution
- What external/domain consequences occurred
- How the case was reconciled
- Who closed the case

---

## 44.2 Auditable Activities

Audit evidence shall be generated for:

- Case creation
- Case modification
- Submission
- Assessment
- Eligibility decision
- Recommendation
- Committee review
- Committee recommendation
- Approval
- Return
- Rejection
- Authorization
- Authorization revocation
- Disposal execution
- Destruction
- Scrap handover
- Sale/auction handover
- Donation
- External transfer-out
- Write-off authorization
- Inventory adjustment request
- J06 outcome recording
- J08 outcome recording
- Finance outcome recording
- Exception recording
- Manual intervention
- Reconciliation
- Cancellation
- Completion
- Closure
- Archival

---

## 44.3 Audit Record Requirements

Audit evidence should preserve, where applicable:

- Actor/System Identity
- Tenant ID
- Institution ID
- Timestamp
- Disposal Case ID
- Disposal Case Number
- Disposal Line ID
- Inventory Item Reference
- Asset Reference
- Operation
- Previous State
- Resulting State
- Authorization Context
- Approval Reference
- Execution Reference
- Correlation ID
- Causation ID
- Reason/Justification
- External Domain Reference

---

## 44.4 Evidence Immutability

Finalized evidence shall be immutable.

Corrections shall be represented as supplemental evidence.

The architecture shall preserve:

```text
Original Evidence
       +
Correction / Supplemental Evidence
       +
Actor
       +
Reason
       +
Timestamp
       =
Complete Historical Reconstruction
```

---

# 45. Observability & Operational Monitoring

J10 shall expose operational visibility sufficient for authorized operators to determine:

- Number of active disposal cases
- Cases awaiting assessment
- Cases awaiting committee review
- Cases awaiting approval
- Approved cases awaiting execution
- Partially executed cases
- Inventory adjustments pending
- Asset retirements pending
- Finance processing pending
- Reconciliation exceptions
- Cases ready for closure
- Long-running cases
- Failed integration attempts

---

## 45.1 Correlation & Traceability

Material multi-domain interactions shall preserve:

| Attribute | Purpose |
|---|---|
| Disposal Case ID | Canonical J10 case identity |
| Correlation ID | Links complete business interaction |
| Causation ID | Identifies initiating request/event |
| Tenant ID | Preserves tenant isolation |
| Institution ID | Preserves institution context |
| Inventory Item Reference | Links inventory context |
| Asset Reference | Links asset context where applicable |
| Authorization Reference | Links approved execution scope |
| Execution Reference | Links physical disposal evidence |
| Adjustment Reference | Links J06 processing |
| Event ID | Supports event deduplication |
| Aggregate Version | Supports ordering/concurrency |

---

## 45.2 Operational Alerts

Alert conditions may include:

- Approval SLA exceeded
- Authorization nearing expiry
- Execution overdue
- Disposal quantity mismatch
- Missing mandatory evidence
- J06 adjustment failure
- J08 retirement failure
- Finance processing failure
- Reconciliation backlog
- Duplicate execution attempt
- Unauthorized operation attempt
- Closure blocked by unresolved exception

Alerts shall use enterprise notification infrastructure.

---

# 46. Performance, Scalability & Resilience

## 46.1 Performance Principle

Normal read operations shall not require synchronous calls to every external bounded context.

Read models may maintain authorized projections of required reference data while preserving canonical ownership.

---

## 46.2 Scalability

J10 shall support workload partitioning by:

- Tenant
- Institution
- Campus
- Disposal Case
- Disposal type
- Processing state

High-volume disposal activity in one institution shall not block unrelated institutions.

---

## 46.3 Idempotency

Retryable operations shall support idempotency.

Duplicate requests shall not create:

- Duplicate cases
- Duplicate approvals
- Duplicate authorizations
- Duplicate physical execution records
- Duplicate inventory adjustment requests
- Duplicate asset retirement requests
- Duplicate financial consequences

---

## 46.4 Retry Architecture

Only recoverable technical failures shall be automatically retried.

```text
Failure
   ¦
   +--------? Recoverable
   ¦              ¦
   ¦              ?
   ¦          Retry Policy
   ¦              ¦
   ¦       +-------------+
   ¦       ¦             ¦
   ¦    Success       Exhausted
   ¦       ¦             ¦
   ¦       ?             ?
   ¦    Continue      Exception
   ¦
   +--------? Business Failure
                  ¦
                  ?
             No Blind Retry
```

Business-rule rejection shall not be treated as transient technical failure.

---

## 46.5 Recovery

Following infrastructure recovery, J10 shall be capable of:

- Resuming pending integration publication
- Reprocessing eligible failed messages
- Rebuilding read projections
- Reconciling uncertain downstream outcomes
- Identifying cases requiring manual review

Recovery shall preserve original case identity and audit lineage.

---

# 47. Configuration & Extensibility

Institutional variation shall be represented through governed configuration rather than hard-coded behavior.

Configuration may govern:

- Disposal types
- Disposal reasons
- Write-off reasons
- Condition classifications
- Disposal methods
- Materiality thresholds
- Committee requirements
- Quorum rules
- Approval authority
- Witness requirements
- Evidence requirements
- Authorization validity
- Reconciliation requirements
- Closure requirements
- Notification thresholds
- Retention policies

Configuration shall not weaken mandatory canonical ownership boundaries.

---

# 48. Architectural Decision Records

### ADR-J10-001 — J10 Owns Disposal Workflow, Not Inventory Balance

**Decision:** J10 owns disposal/write-off lifecycle, authorization, execution evidence, and reconciliation state.

J06 remains authoritative for inventory quantities.

**Rationale:** Disposal governance and inventory accounting are separate consistency boundaries.

---

### ADR-J10-002 — Physical Disposal and Inventory Adjustment Are Separate Facts

**Decision:** Confirmation of physical disposal does not directly modify canonical inventory balances.

**Rationale:** J10 records physical/business execution while J06 controls inventory state.

---

### ADR-J10-003 — Disposal Execution Is Immutable

**Decision:** Confirmed disposal, destruction, or handover records shall not be destructively edited.

**Rationale:** Physical disposal is an irreversible institutional event requiring permanent evidence.

---

### ADR-J10-004 — J09 Findings May Initiate but Do Not Own Write-off

**Decision:** Inventory audit findings may trigger J10 processing, but formal disposal/write-off authorization remains owned by J10.

**Rationale:** Audit verification and disposal authorization are separate business responsibilities.

---

### ADR-J10-005 — Asset Retirement Remains Owned by J08

**Decision:** J10 publishes asset-linked disposal facts but does not directly retire canonical assets.

**Rationale:** Asset lifecycle integrity remains within J08.

---

### ADR-J10-006 — Financial Posting Remains Outside J10

**Decision:** J10 provides disposal/write-off facts to Finance but shall not create canonical ledger postings.

**Rationale:** Accounting ownership remains with Finance.

---

### ADR-J10-007 — Authorization Precedes Execution

**Decision:** Physical disposal/write-off execution requires explicit valid authorization.

**Rationale:** Inventory shall not leave institutional custody without governed authority.

---

### ADR-J10-008 — Segregation of Duties Is Policy-Driven

**Decision:** J10 shall provide sufficient business context for authorization/workflow services to enforce institutional segregation-of-duties policies.

**Rationale:** Disposal is a fraud-sensitive and financially material business process.

---

### ADR-J10-009 — Reconciliation Is Required for Multi-Domain Completion

**Decision:** Physical execution alone does not necessarily complete a disposal case.

**Rationale:** Inventory, asset, and financial consequences may require confirmed downstream processing.

---

### ADR-J10-010 — Disposal Evidence Is Permanent Business Evidence

**Decision:** Finalized committee decisions, approvals, execution records, certificates, and reconciliation evidence shall remain immutable.

**Rationale:** Institutional disposal must remain reconstructable for audit and compliance.

---

# 49. Architecture Completion Criteria

L2-J10 shall be considered architecturally complete when:

- Disposal/write-off cases possess canonical J10 identity.
- Disposal lines preserve inventory references and quantities.
- J06 remains authoritative for inventory balances.
- J08 remains authoritative for asset lifecycle.
- J09 remains authoritative for inventory audit findings.
- Finance remains authoritative for accounting.
- Eligibility assessment is explicitly modeled.
- Disposal recommendations are explicitly modeled.
- Committee governance is supported where required.
- Approval and authorization are distinct from execution.
- Disposal cannot execute without valid authorization.
- Partial execution remains traceable.
- Confirmed physical disposal is immutable.
- Destruction evidence is preserved.
- Scrap handover is governed.
- Sale/auction disposal preserves appropriate references.
- Donation and external transfer-out are governed.
- Loss/write-off processing requires sufficient evidence.
- Inventory adjustment occurs through J06.
- Asset retirement occurs through J08.
- Financial consequences occur through Finance.
- Multi-domain outcomes are reconciled.
- Exceptions cannot silently disappear.
- Segregation of duties can be enforced.
- Commands support appropriate idempotency.
- Domain events are immutable and reliably published.
- Tenant and institution isolation are preserved.
- Audit evidence reconstructs the complete disposal lifecycle.
- Closed and archived cases remain immutable.

---

# 50. Final Architecture Statement

**L2-J10 — Disposal, Scrap & Write-Off** establishes the canonical institutional governance boundary for terminal inventory disposition within SutraERP.

J10 governs:

- Disposal/write-off initiation
- Eligibility assessment
- Condition evidence
- Disposal recommendation
- Materiality review
- Committee review
- Approval
- Authorization
- Physical disposal
- Destruction
- Scrap handover
- Sale/auction disposal evidence
- Donation
- External transfer-out
- Inventory loss write-off
- Disposal evidence
- Multi-domain reconciliation
- Disposal completion and closure

The architecture preserves the fundamental ownership separation:

```text
Inventory / Audit / Asset Requirement
              ¦
              ?
     Disposal, Scrap & Write-Off (J10)
              ¦
              ¦ Assessment
              ¦ Recommendation
              ¦ Approval
              ¦ Authorization
              ?
       Disposal Execution
              ¦
       +------+-----------+
       ¦      ¦           ¦
       ?      ?           ?
      J06    J08       Finance
   Inventory Asset     Accounting
    State   Lifecycle
       ¦      ¦           ¦
       +------+-----------+
              ¦
              ?
        Reconciliation
              ¦
              ?
       Completion / Closure
```

J10 therefore answers:

> **Why was institutional inventory disposed of or written off, who authorized that decision, what exactly was executed, and what evidence proves the final outcome?**

J06 answers:

> **What is the authoritative inventory state after the resulting inventory transaction?**

J08 answers:

> **What is the authoritative lifecycle state of any associated institutional asset?**

Finance answers:

> **What accounting consequence results from the disposal or write-off?**

J09 answers:

> **What did inventory verification or audit establish before disposal/write-off processing began?**

This separation prevents duplicate systems of record while providing a complete, auditable, secure, policy-driven, and extensible disposal architecture for SutraERP.

---

**End of L2-J10 — Disposal, Scrap & Write-Off**
