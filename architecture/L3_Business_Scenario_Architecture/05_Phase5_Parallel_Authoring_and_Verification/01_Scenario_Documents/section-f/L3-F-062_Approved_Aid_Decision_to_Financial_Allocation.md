# Approved Aid Decision to Financial Allocation

**Scenario ID:** L3-F-062  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Scholarship & Financial Aid  
**Participating Domains:** C — Student Lifecycle; F — Finance  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-F00 — Finance Domain Model & Foundation; L2-F11 — Scholarship & Financial Aid; L2-C07 — Scholarship Eligibility Workflows; L2-F01 — Student Charge & Fee Management; L2-F07 — Accounts Receivable  
**Referenced By:** Student Charge & Fee Management; Accounts Receivable; Financial Reporting; Audit; Executive Dashboards; Notification as applicable

## 2. Scenario Purpose

Govern the business outcome from an approved scholarship award to the point at which the award's financial assistance is allocated and the authoritative downstream financial consequence is available for the owning Finance processes.

The scenario deliberately separates financial assistance governance from eligibility determination, student charge generation, receivable creation, cash collection, and accounting. Scholarship & Financial Aid owns the financial assistance decision and Benefit Allocation; downstream Finance domains determine the consequences within their own bounded contexts.

## 3. Business Outcome

A scholarship is established against an authoritative eligibility outcome, approved, allocated to its intended institutional financial context, associated with a valid funding source, and published as immutable business facts so that Student Charge & Fee Management and other downstream Finance consumers can determine their own business consequences.

A Scholarship & Financial Aid outcome does **not** itself create a Student Charge, receivable, payment, treasury transaction, or journal entry.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an authoritative eligibility outcome from L2-C07 supports consideration of a scholarship award.

### 4.2 End Boundary

The scenario ends when the Financial Aid Aggregate has recorded the approved scholarship, allocated the approved benefit, associated the required funding source, and published the canonical downstream business facts that enable the owning financial domains to continue their own processes.

### 4.3 In Scope

- Consumption of the authoritative eligibility outcome.
- Registration of the scholarship/financial assistance.
- Recording of the approval decision.
- Benefit allocation.
- Funding source association.
- Activation of the approved assistance.
- Publication of `FinancialAidApproved`, `BenefitAllocated`, `FundingSourceAssigned`, and `FinancialAidActivated` where the corresponding state transitions occur.
- Downstream business hand-off to Student Charge & Fee Management.

### 4.4 Out of Scope

- Eligibility policy evaluation.
- Student record ownership.
- Fee Structure ownership.
- Student Charge creation or modification.
- Accounts Receivable creation.
- Payment collection.
- Treasury operations.
- General Ledger posting.
- Technical API, database, messaging, queue, retry, or infrastructure behaviour.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Canonical Ownership |
|---|---|---|
| C — Student Lifecycle | Provides the student context and authoritative eligibility outcome produced by the Scholarship Eligibility context. | Student and eligibility concepts remain C-owned. |
| F — Scholarship & Financial Aid | Owns the scholarship/financial assistance decision, Benefit Allocation, Funding Source Assignment, approval history and utilization/renewal lifecycle. | Financial Aid Aggregate remains F11-owned. |
| F — Student Charge & Fee Management | Consumes approved assistance facts and independently determines how they affect charges. | Student Charge and Fee Structure remain F01-owned. |
| F — Accounts Receivable | Independently determines receivable consequences after charge processing. | Receivable remains F07-owned. |

L2-F11 explicitly states that eligibility is owned by L2-C07 and that the Scholarship & Financial Aid context owns the institutional financial assistance decision, while Student Charge, Accounts Receivable, Treasury, General Ledger and Budget remain external. fileciteturn217file0

## 6. Business Actors, Roles and Responsibilities

L2-F11 explicitly records that an Actors section is not present. Therefore this scenario does not invent a scholarship officer, finance administrator, approver, student role, or disbursement officer.

Business responsibilities remain capability-owned:

- L2-C07 provides the authoritative eligibility outcome.
- Scholarship & Financial Aid records the financial assistance decision and allocation.
- Downstream Finance contexts apply their own business consequences.

Authorization logic remains externally governed by the applicable Authorization & Policy architecture. fileciteturn217file0

## 7. Preconditions and Entry Criteria

- The beneficiary student exists in the authoritative student context.
- An authoritative eligibility outcome exists.
- The eligibility outcome supports scholarship consideration.
- The scholarship/financial assistance can be registered against the Financial Aid Aggregate.
- A valid approval decision can be recorded.
- A valid Funding Source can be associated before activation where required.
- The requested Benefit Allocation complies with the Allocation invariants.
- The downstream Student Charge context can consume the approved business fact.

L2-F11 requires eligibility to exist before approval and approval before Benefit Allocation. fileciteturn222file0

## 8. Trigger

An authoritative scholarship-eligibility outcome indicates that the student is eligible or conditionally eligible for scholarship consideration.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | The authoritative eligibility outcome is consumed without re-evaluating eligibility policy. | Scholarship & Financial Aid | A governed eligibility input exists for the financial assistance decision. |
| 2 | The scholarship assistance is registered within the Financial Aid Aggregate for the beneficiary student. | Scholarship & Financial Aid | Financial assistance enters the canonical lifecycle. |
| 3 | The eligibility outcome is recorded against the Financial Aid Aggregate. | Scholarship & Financial Aid | Eligibility history supporting the decision is preserved. |
| 4 | The scholarship approval decision is recorded. | Scholarship & Financial Aid | The assistance reaches `Approval Recorded` and satisfies the approval prerequisite for allocation. |
| 5 | The approved scholarship benefit is allocated according to applicable Allocation Policy and the approved assistance terms. | Scholarship & Financial Aid | `Benefit Allocated` becomes an authoritative business fact. |
| 6 | The applicable Funding Source is associated with the allocated benefit. | Scholarship & Financial Aid | Funding responsibility is explicit and traceable. |
| 7 | The scholarship assistance is activated through the canonical Financial Aid Aggregate lifecycle. | Scholarship & Financial Aid | Assistance reaches `Active`. |
| 8 | Scholarship & Financial Aid publishes the canonical business events corresponding to the completed transitions. | Scholarship & Financial Aid | Downstream Finance consumers receive immutable facts. |
| 9 | Student Charge & Fee Management consumes the published scholarship/benefit allocation fact and independently determines the applicable charge consequence. | Student Charge & Fee Management | The financial obligation owner can apply its own business rules. |
| 10 | Any later receivable consequence is determined by Accounts Receivable after its own charge/receivable responsibilities are satisfied. | Accounts Receivable | Receivable ownership remains separate from financial assistance ownership. |

L2-F11 defines the Financial Aid Aggregate lifecycle as `Registered → Eligibility Recorded → Approval Recorded → Benefit Allocated → Active`, with approval required before allocation and allocation required before activation. fileciteturn221file0turn222file0

## 10. Alternate Business Flows

### A1 — Conditional Eligibility

If L2-C07 publishes a conditionally eligible outcome, Scholarship & Financial Aid consumes the authoritative outcome and applies the applicable financial assistance policy. It does not re-evaluate the eligibility decision.

### A2 — Approval Rejected

If the financial assistance approval decision is rejected, the rejected assistance does not progress to `Benefit Allocated` or `Active`. L2-F11 explicitly requires that rejected assistance cannot become Active. fileciteturn222file0

### A3 — Funding Source Not Available

If no valid Funding Source can be associated for the approved allocation, the assistance cannot progress through a valid activation outcome. No unsupported funding source is invented.

### A4 — Benefit Allocation Requires Later Charge Context

The Benefit Allocation may be recorded as business intent while Student Charge & Fee Management independently determines how that allocation affects charges. The allocation does not itself generate a financial obligation. fileciteturn220file0

### A5 — Existing Assistance Already Exists

A new scholarship/assistance business fact is created only through the applicable Financial Aid Aggregate lifecycle. Historical approvals and allocations are not overwritten or reassigned.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Resolution |
|---|---|
| No authoritative eligibility outcome exists | Do not record the financial assistance approval; obtain the authoritative eligibility fact from L2-C07. |
| Eligibility is rejected | Do not progress the assistance into approval/allocation/active state. |
| Approval is rejected | Preserve the rejected financial assistance decision; do not allocate or activate the benefit. |
| Benefit allocation violates an invariant | Reject the allocation business action; preserve existing Aggregate state. |
| Funding Source is invalid or unavailable | Do not treat the allocation as fully governed until a valid Funding Source is associated where required. |
| Student Charge consequence cannot yet be determined | Preserve the approved financial assistance facts and allow Student Charge Management to resolve its own business consequence later. |
| Downstream domain fails to react | Completed Financial Aid facts remain valid; downstream recovery occurs through subsequent business processing, not by mutating the Financial Aid Aggregate. |

L2-F11 states that neighbouring failures shall not compromise Aggregate consistency, and recovery occurs through subsequent business events. fileciteturn224file0

## 12. Business Rules and Decision Constraints

The scenario applies these authoritative L2-F11 rules:

- Eligibility Assessment must exist before approval is recorded.
- Eligibility history is immutable.
- Approval Decision must exist before Benefit Allocation.
- Approval history is immutable.
- Rejected assistance cannot become Active.
- Benefit Allocation requires an approved assistance decision.
- Allocation history is immutable.
- Benefit Allocation shall not generate financial obligations.
- Every allocated benefit references one valid Funding Source.
- Utilization requires active assistance.
- Historical business facts are never deleted, overwritten, reassigned, or repurposed.
- Eligibility outcomes are externally evaluated.
- Authorization outcomes are externally determined.
- The Financial Aid Aggregate is the sole authoritative consistency boundary for institutional financial assistance. fileciteturn222file0turn217file0

The scenario does not invent a local eligibility rule, scholarship approval policy, fee rule, receivable rule, or accounting rule.

## 13. State and Lifecycle Impact

The authoritative Financial Aid Aggregate lifecycle is:

`Registered → Eligibility Recorded → Approval Recorded → Benefit Allocated → Active`

Relevant state transitions are:

| Current State | Permitted Transition |
|---|---|
| Registered | Eligibility Recorded |
| Eligibility Recorded | Approval Recorded |
| Approval Recorded | Benefit Allocated |
| Benefit Allocated | Active |

Transitions outside this lifecycle are rejected. fileciteturn222file0

The scenario does not introduce a new disbursement state. In L2-F11, the business responsibility ends with the financial assistance decision and allocation; downstream contexts independently determine the financial consequences. fileciteturn217file0turn224file0

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

The Financial Aid Aggregate publishes the following relevant completed business facts:

- `EligibilityAssessmentRecorded`
- `FinancialAidApproved`
- `ApprovalDecisionRecorded`
- `BenefitAllocated`
- `FundingSourceAssigned`
- `FinancialAidActivated`

L2-F11 explicitly defines these event categories and requires events to represent immutable completed business facts. fileciteturn222file0turn223file0

### 14.2 Downstream Business Hand-offs

| From | To | Business Fact / Hand-off | Purpose |
|---|---|---|---|
| L2-C07 | Scholarship & Financial Aid | `EligibilityGranted`, `EligibilityConditionallyGranted`, or `EligibilityRejected` | Provides the authoritative eligibility result; C07 retains eligibility ownership. |
| Scholarship & Financial Aid | Student Charge & Fee Management | `FinancialAidApproved`, `BenefitAllocated`, and applicable assistance lifecycle facts | Enables F01 to determine how approved assistance affects student charges. |
| Scholarship & Financial Aid | Accounts Receivable | `BenefitAllocated`, `FinancialAidRevoked`, `FinancialAidExpired` as applicable | Provides receivable context after F01 has applied its charging rules. |
| Scholarship & Financial Aid | Financial Reporting / Audit | Approved, allocation, funding and utilization facts | Supports reporting and historical reconstruction. |

L2-F11 explicitly maps `FinancialAidApproved` and `BenefitAllocated` to Student Charge & Fee Management and states that Fee Management determines the financial obligation independently. fileciteturn223file0turn224file0

Events remain distinct from business contracts and synchronous hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Business Authority

L2-F11 records approval outcomes inside the Financial Aid Aggregate but states that authorization remains externally governed. The scenario therefore does not invent an approving role or approval hierarchy. fileciteturn220file0turn222file0

### 15.2 Separation of Duties

No scholarship-specific separation-of-duty rule or human-role catalogue is defined in L2-F11. No new rule is introduced.

### 15.3 Audit and Evidence

The scholarship award must remain historically traceable through:

- student reference;
- eligibility outcome;
- scholarship/benefit identity;
- approval decision;
- allocation;
- Funding Source;
- effective period;
- activation;
- published event history.

L2-F11 requires immutable approval history, allocation history, funding-source history and permanently reconstructable Domain Events. fileciteturn222file0turn223file0

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Authoritative eligibility outcome has been consumed.
- Scholarship assistance has been registered.
- Eligibility outcome has been recorded.
- Approval decision has been recorded.
- Benefit Allocation has been established.
- Valid Funding Source has been associated.
- Financial assistance has become `Active`.
- Canonical downstream events have been published.
- Student Charge & Fee Management can independently apply the approved assistance to its own charge process.

### 16.2 Postconditions

- Financial Aid Aggregate is in `Active`.
- Approval and allocation history remains immutable.
- The scholarship remains owned by Scholarship & Financial Aid.
- Student Charge and Accounts Receivable remain owned by their respective bounded contexts.
- No financial obligation, receivable, payment, treasury transaction or journal entry is created by this scenario.
- Downstream domains can consume authoritative financial assistance facts.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

L2-F11 explicitly records an **Actors gap** in its canonical crosswalk. Therefore no human actor or specific approval authority is named in this L3. fileciteturn217file0

The term **“disbursement”** in the frozen scenario title is not a named lifecycle state in L2-F11. The L2 architecture defines approval, Benefit Allocation and activation, then states that downstream domains consume the financial assistance decision and independently determine their own financial behaviour. Accordingly, this L3 interprets the scenario's governed disbursement outcome as the completed assistance/allocation fact that enables downstream financial processing; it does not invent a new disbursement state, event, payment process, or treasury responsibility. fileciteturn217file0turn220file0turn224file0

### 17.2 Assumptions

No assumptions override locked architecture.

### 17.3 Deferred Questions

Any future requirement to model an explicit outbound scholarship cash disbursement would require an upstream decision identifying the canonical owner, lifecycle, events, and relationship to Payment/Treasury before being introduced into L3.
