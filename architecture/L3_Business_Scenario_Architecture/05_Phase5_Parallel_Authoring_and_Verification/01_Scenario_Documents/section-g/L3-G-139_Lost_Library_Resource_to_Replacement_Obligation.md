# Lost Library Resource to Replacement Obligation

**Scenario ID:** L3-G-139  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-139 |
| Canonical Title | Lost Library Resource to Replacement Obligation |
| Primary Owner | Library Management |
| Participating Domains | G, F, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the complete institutional outcome in which a borrowed library resource is determined to be lost, the loss is recorded against the relevant circulation/resource context, the applicable replacement obligation is established under library policy, and the resulting financial/communication consequences are handed to the appropriate participating domains.

The L2 return process explicitly supports Lost Resource Information, and Fine Management supports a Lost Resource fine category and Replacement Charge.

Library Management owns operational loss handling and library fine/replacement obligation calculation. Finance owns accounting/payment execution.

## 3. Business Outcome

A Library Copy/resource has an authoritative lost-resource outcome, the member's associated replacement obligation has been established under the applicable policy, the library fine/replacement charge is recorded, and Finance/H can act on the resulting business information.

The scenario does not execute payment, procure a replacement, or invent a replacement amount.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A borrowed library resource is reported or determined to be lost.

### 4.2 End Boundary

The loss is recorded and the applicable replacement obligation/fine outcome is established.

### 4.3 In Scope

- Identify active borrowing.
- Identify member and Library Copy/resource.
- Establish loss condition.
- Record Lost Resource Information.
- Determine applicable replacement obligation.
- Apply configurable fine/replacement policy.
- Record replacement charge/fine.
- Preserve circulation and loss history.
- Provide Finance reference.
- Provide H notification outcome.

### 4.4 Out of Scope

- Payment execution.
- Accounting settlement.
- Procurement of replacement material.
- New resource acquisition.
- Physical repair.
- Membership closure.
- Technical workflows or infrastructure.

## 5. Ownership and Participating Domains

### Library Management — G

Owns circulation, Library Copy operational handling, Fine Management and the replacement-obligation business outcome.

### Finance — F

Owns accounting/payment processing.

### Communication — H

Owns notification delivery.

Neither F nor H becomes owner of the lost-resource business outcome through participation.

## 6. Business Actors, Roles and Responsibilities

### Library Member

Reports or is identified as responsible for the lost resource under the applicable institutional policy.

### Librarian / Circulation Authority

Verifies and records the loss and applies the applicable library policy.

### Finance

Receives the financial reference and processes accounting/payment consequences.

### H

Communicates the lost-resource/fine outcome where required.

## 7. Preconditions and Entry Criteria

1. A Library Copy/resource can be identified.
2. The relevant borrowing or member relationship can be established where applicable.
3. The loss condition can be evaluated.
4. Applicable lost-resource/replacement policy is available.
5. The library fine/replacement process is available.

## 8. Trigger

A borrowed library resource is reported or determined to be lost.

## 9. Main Business Flow

### Phase A — Establish Loss

1. Identify the resource and associated Library Copy.
2. Identify the member and active borrowing where applicable.
3. Verify that the resource is not available for normal return.
4. Establish the lost-resource condition.
5. Record Lost Resource Information.
6. Preserve the relevant circulation history.

### Phase B — Determine Replacement Obligation

7. Determine the applicable resource category.
8. Determine the applicable membership/institutional policy context.
9. Determine whether a replacement charge or other fine category applies.
10. Apply the configurable library fine/replacement policy.
11. Establish the replacement obligation.
12. Record the applicable fine/charge category.

### Phase C — Complete Business Outcome

13. Record the lost-resource/fine outcome.
14. Provide the Finance reference where financial processing is required.
15. Provide the notification outcome to H where required.
16. Preserve the complete loss and obligation history.
17. End the scenario.

## 10. Alternate Business Flows

### 10.1 Resource Is Found Before Loss Is Finalised

The loss outcome is not finalised and the resource follows the applicable return/condition process.

### 10.2 Resource Is Lost After the Borrowing Due Date

The applicable overdue and lost-resource policies are applied without inventing whether charges are cumulative or mutually exclusive.

### 10.3 Replacement Charge Applies

A Replacement Charge is recorded as the applicable fine category under policy.

### 10.4 Lost Resource Fine Applies

The Lost Resource category is used where the authoritative fine policy requires it.

### 10.5 No Replacement Obligation Applies

If policy establishes no charge, no unsupported financial obligation is created; the lost-resource condition remains recorded.

### 10.6 Finance Processing Required

G establishes the library obligation/reference; F performs accounting/payment processing.

## 11. Failure, Interruption and Business Recovery

### 11.1 Loss Cannot Be Established

The resource remains under its current authoritative circulation/condition outcome until the business loss decision can be established.

### 11.2 Member/Resource Relationship Cannot Be Established

The loss is not assigned to an unsupported party. Processing waits for authoritative relationship information.

### 11.3 Replacement Rule Cannot Be Established

No invented replacement value is calculated. The loss remains recorded while the applicable policy is resolved.

### 11.4 Finance Is Not Yet Ready

The G library obligation remains authoritative; Finance processing is downstream.

### 11.5 Notification Is Required

H owns communication delivery; no technical retry or queue mechanism is defined.

## 12. Business Rules and Decision Constraints

Fine categories may include:

- Lost Resource.
- Replacement Charge.
- Overdue Fine.
- Damaged Resource.
- Administrative Penalty.

Categories remain configurable.

Fine calculation may consider:

- Resource Category.
- Membership Category.
- Grace Period.
- Number of Days.
- Institutional Policies.

Calculation rules remain configurable.

Return processing may record Lost Resource Information, and historical circulation records remain immutable.

No replacement amount, depreciation rule, ownership-liability rule, or payment deadline is invented.

## 13. State and Lifecycle Impact

The Fine lifecycle is:

`Calculated → Pending → Paid / Waived → Closed`.

The scenario establishes the calculated/pending financial obligation where a charge applies.

The Circulation lifecycle remains:

`Issued → Active → Renewed → Returned → Closed`.

The loss outcome must not be represented as an invented circulation state.

The Library Copy lifecycle remains authoritative and may subsequently require an operational withdrawal/condition decision according to the Library Copy lifecycle and applicable policy.

## 14. Business Events and Cross-Domain Collaboration

The available L2 service model explicitly supports Fine Services including Calculate Fine and Query Outstanding Fine, and Acquisition Services for introducing replacement resources, but procurement remains outside Library Management.

The available event material does not expose a dedicated canonical lost-resource event in the retrieved source. Therefore this draft does not invent `ResourceLost` or `ReplacementObligationCreated`.

### Cross-domain collaboration

- **F:** receives the financial reference and handles accounting/payment.
- **H:** handles notification delivery.
- **G:** owns the loss determination and library obligation.

Replacement procurement/acquisition is outside the scope of this scenario.

## 15. Business Authorization, Approval and Audit Obligations

Loss determination and obligation calculation follow configured Library Management authorization.

The loss record, member/resource relationship, reason/observation, applicable policy basis, replacement obligation and resulting financial reference must be auditable.

Where institutional policy requires administrative approval for an exceptional obligation or waiver, that approval remains governed by the appropriate policy.

No technical access control/IAM implementation is specified.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Resource identified.
- Member/borrowing relationship established where applicable.
- Loss condition established.
- Lost Resource Information recorded.
- Applicable replacement/fine policy evaluated.
- Replacement obligation calculated where applicable.
- Financial reference established.
- H notification outcome made available where applicable.
- History preserved.

### Postconditions

- Resource is recorded as lost within the applicable Library Management business context.
- Replacement/fine obligation is authoritative where applicable.
- Fine proceeds through its canonical lifecycle.
- Finance owns payment/accounting.
- H owns notification.
- Any physical replacement/acquisition follows a separate acquisition process.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

The retrieved L2 does not expose a dedicated canonical lost-resource event. No event is invented.

### Assumptions

- Replacement/fine calculation rules remain configurable.
- Loss determination is a Library Management business decision.
- Procurement/replacement acquisition remains outside this scenario.

### Deferred Questions

Confirm the final canonical event vocabulary for lost-resource outcomes before Architecture Approval/Lock if an event is mandatory.
