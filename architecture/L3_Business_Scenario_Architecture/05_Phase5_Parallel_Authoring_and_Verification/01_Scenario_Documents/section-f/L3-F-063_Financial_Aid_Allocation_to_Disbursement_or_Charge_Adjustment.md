# Financial Aid Allocation to Disbursement or Charge Adjustment

**Scenario ID:** L3-F-063  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Scholarship & Financial Aid  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-F11 Scholarship & Financial Aid; L2-C07 Scholarship Eligibility Workflows; L2-F01 Student Charge & Fee Management; L2-F07 Accounts Receivable  
**Referenced By:** Student Charge & Fee Management; Accounts Receivable; Financial Reporting; Audit; Executive Dashboards

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-F-063 |
| Canonical Title | Financial Aid Allocation to Disbursement or Charge Adjustment |
| Primary Owner | Scholarship & Financial Aid |
| Participating Domains | C, F |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern the business consequence of an approved financial-aid allocation by publishing the authoritative allocation fact and enabling the canonical downstream financial domain to determine whether the assistance results in a charge adjustment or another governed financial consequence.

Scholarship & Financial Aid owns the assistance decision and allocation. It does not own Student Charges, Receivables, Payments, Treasury, or General Ledger.

## 3. Business Outcome

An approved financial-aid allocation is recorded against the Financial Aid Aggregate, associated with its valid funding source, and published as an immutable business fact. Student Charge & Fee Management then independently determines the applicable charge adjustment or other charge consequence under its own ownership.

The allocation itself does not create a financial obligation.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when approved financial assistance is eligible for Benefit Allocation.

### 4.2 End Boundary

The scenario ends when the allocation has become an authoritative business fact and the downstream Student Charge & Fee Management context has the business fact required to determine its own charge consequence.

### 4.3 In Scope

- Approved assistance allocation.
- Funding Source association.
- Activation of assistance.
- Publication of `BenefitAllocated`.
- Publication of related approved/activation facts.
- Downstream consumption by Student Charge & Fee Management.
- Charge-adjustment consequence remaining under F01 ownership.

### 4.4 Out of Scope

- Eligibility evaluation.
- Approval authority.
- Student Charge creation.
- Receivable creation.
- Payment or cash disbursement execution.
- Treasury.
- General Ledger.
- Technical integration mechanics.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Ownership |
|---|---|---|
| Scholarship & Financial Aid | Owns Financial Aid Aggregate, Benefit Allocation and Funding Source assignment. | Canonical assistance ownership. |
| Student Lifecycle / Scholarship Eligibility | Provides authoritative eligibility facts where applicable. | Eligibility remains external. |
| Student Charge & Fee Management | Determines how approved assistance affects student charges. | Charge ownership remains external. |
| Accounts Receivable | Determines receivable consequences after charge processing. | Receivable ownership remains external. |

L2-F11 explicitly separates financial assistance governance from charges, receivables, cash collection and accounting.

## 6. Business Actors, Roles and Responsibilities

L2-F11 records an Actors-section gap and does not define human roles. This scenario therefore does not invent a financial-aid officer, cashier, approver, bursar, or disbursement role.

Capability responsibilities remain authoritative: Scholarship & Financial Aid governs the assistance; Student Charge & Fee Management governs charge consequences.

## 7. Preconditions and Entry Criteria

- Assistance has a valid approval decision.
- Eligibility prerequisites have been satisfied.
- The Benefit Allocation complies with allocation invariants.
- A valid Funding Source is available.
- The Financial Aid Aggregate is in `Approval Recorded`.
- The downstream charge context can consume the published assistance fact.

L2-F11 requires Approval Decision before Benefit Allocation and a valid Funding Source for every allocated benefit.

## 8. Trigger

An approved financial-aid decision becomes ready for Benefit Allocation.

## 9. Main Business Flow

| Step | Business Behaviour | Owner | Result |
|---|---|---|---|
| 1 | Confirm the assistance has an authoritative approval decision. | Scholarship & Financial Aid | Allocation is permitted. |
| 2 | Allocate the approved financial benefit according to the applicable Allocation Policy. | Scholarship & Financial Aid | `Benefit Allocated` becomes the canonical allocation state. |
| 3 | Associate the valid Funding Source. | Scholarship & Financial Aid | Funding responsibility is traceable. |
| 4 | Activate the assistance when the canonical lifecycle permits activation. | Scholarship & Financial Aid | Assistance reaches `Active`. |
| 5 | Publish `BenefitAllocated` and the related completed lifecycle business facts. | Scholarship & Financial Aid | Downstream consumers receive immutable facts. |
| 6 | Student Charge & Fee Management consumes the allocation fact. | Student Charge & Fee Management | F01 determines its own charge consequence. |
| 7 | Where applicable, F01 applies the assistance according to its charge rules and produces its own charge adjustment/business outcome. | Student Charge & Fee Management | Charge ownership remains with F01. |
| 8 | Where a receivable consequence follows from charge processing, Accounts Receivable independently applies its own rules. | Accounts Receivable | Receivable ownership remains with F07. |

L2-F11 explicitly maps `BenefitAllocated` to Student Charge & Fee Management and states that Fee Management determines how approved assistance influences student charges.

## 10. Alternate Business Flows

### A1 — Allocation Not Permitted

If approval is absent, Benefit Allocation does not occur.

### A2 — Funding Source Invalid

If a valid Funding Source cannot be associated, the allocation cannot be treated as fully governed.

### A3 — Assistance Produces a Charge Adjustment

Student Charge & Fee Management independently determines and records the charge consequence. Scholarship & Financial Aid does not create or modify the Student Charge.

### A4 — No Charge Adjustment Is Required

The published allocation remains an authoritative assistance fact. Absence of a charge consequence does not transfer ownership of the allocation.

### A5 — Downstream Charge Processing Is Later

The Financial Aid Aggregate remains authoritative for the approved allocation. Downstream processing can continue through the published business fact without changing the historical assistance decision.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Allocation violates an invariant | Reject the allocation and preserve the existing Aggregate state. |
| Funding Source is invalid | Do not complete the governed allocation outcome. |
| Downstream charge processing is incomplete | Preserve the completed Financial Aid fact; F01 resolves its own business processing. |
| Downstream consumer has a business failure | Do not mutate the Financial Aid Aggregate to compensate; subsequent business facts govern recovery. |
| A later reversal/revocation becomes necessary | Use the applicable Financial Aid revocation/recovery lifecycle while preserving historical allocation facts. |

L2-F11 requires neighbouring failures not to compromise Aggregate consistency and states that recovery occurs through subsequent business events.

## 12. Business Rules and Decision Constraints

- Benefit Allocation requires an approved assistance decision.
- Allocation history is immutable.
- Allocation does not generate financial obligations.
- Every allocated benefit references one valid Funding Source.
- Historical approvals and allocations are never overwritten.
- Authorization outcomes remain externally governed.
- Eligibility outcomes remain externally evaluated.
- Student Charges remain owned by Student Charge & Fee Management.
- Accounts Receivable remains independently owned.
- Scholarship & Financial Aid does not create charges, receivables, payments, treasury transactions or journal entries.

## 13. State and Lifecycle Impact

Relevant Financial Aid lifecycle:

`Registered → Eligibility Recorded → Approval Recorded → Benefit Allocated → Active`

The scenario covers:

`Approval Recorded → Benefit Allocated → Active`

No `Disbursed` or `Charge Adjusted` state is invented inside the Financial Aid Aggregate. The title's alternative downstream outcomes remain owned by the applicable financial domain.

L2-F11 defines `Benefit Allocated` and `Active` as canonical lifecycle states and rejects transitions outside the defined lifecycle.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Events

- `BenefitAllocated`
- `FundingSourceAssigned`
- `FinancialAidActivated`
- `FinancialAidApproved` where the approval transition is part of the scenario's initiating state

The Financial Aid Aggregate is the canonical publisher. Events represent completed immutable business facts.

### 14.2 Cross-Domain Hand-offs

| From | To | Fact | Purpose |
|---|---|---|---|
| Scholarship & Financial Aid | Student Charge & Fee Management | `BenefitAllocated`, `FinancialAidApproved` | Enables F01 to determine charge consequences. |
| Scholarship & Financial Aid | Accounts Receivable | Applicable allocation/revocation/expiry facts | Provides context after F01's charging responsibilities. |
| Scholarship & Financial Aid | Reporting/Audit | Allocation, funding and lifecycle facts | Supports financial visibility and reconstruction. |

Events are distinct from business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

Authorization is externally governed. No human approver is invented.

### 15.2 Approval

An approval decision must exist before allocation. Rejected assistance cannot become Active.

### 15.3 Audit Evidence

The historical record must preserve:

- assistance identity;
- beneficiary reference;
- approval decision;
- Benefit Allocation;
- Funding Source;
- effective period;
- activation;
- published event history;
- downstream references where applicable.

L2-F11 requires immutable approval/allocation/funding history and permanent event traceability.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Approved assistance is allocated.
- Valid Funding Source is associated.
- Assistance is Active where the lifecycle permits activation.
- `BenefitAllocated` and applicable lifecycle facts are published.
- Student Charge & Fee Management has the authoritative fact needed for its own charge determination.

### 16.2 Postconditions

- Financial Aid Aggregate is in `Active`.
- Allocation history is immutable.
- Financial assistance remains F11-owned.
- Any charge adjustment remains F01-owned.
- Any receivable remains F07-owned.
- No payment, treasury transaction or journal entry is created by this scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

The canonical title uses “Disbursement or Charge Adjustment,” but L2-F11 does not define a `Disbursed` state or a cash-disbursement event. F11 instead defines Benefit Allocation and downstream consumption by Student Charge & Fee Management. Therefore no separate cash disbursement workflow is invented here.

### 17.2 Assumptions

No assumption overrides locked architecture.

### 17.3 Deferred Questions

If a true cash-disbursement outcome is later required, upstream architecture must identify its canonical owner, lifecycle, publisher and relationship to Treasury/Banking before it is introduced into L3.
