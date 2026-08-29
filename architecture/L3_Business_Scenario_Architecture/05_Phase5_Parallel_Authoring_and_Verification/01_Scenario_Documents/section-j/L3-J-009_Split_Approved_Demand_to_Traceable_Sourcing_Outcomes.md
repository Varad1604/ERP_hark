# Split Approved Demand to Traceable Sourcing Outcomes

# 1. Scenario Metadata

**Scenario ID:** L3-J-009  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Procurement Planning & Sourcing  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** L3-J-004 through L3-J-007 sourcing-method scenarios; downstream J04 Purchase Order scenarios consuming approved sourcing outcomes  

---
# 2. Scenario Purpose

This scenario governs the deliberate division of one approved procurement demand into multiple sourcing packages or sourcing outcomes when legitimate differences in category, specification, delivery, location, funding, supplier market, risk, method, lot, or timing require separate treatment.

It ensures that splitting improves sourcing fitness without fragmenting value to avoid competition, approval, publication, or oversight. The original demand, every split component, its sourcing event, allocated outcome, and remaining balance must remain traceable and quantity-conserving.

---

# 3. Business Outcome

An approved demand is divided under valid authority into controlled sourcing components, each component reaches a traceable approved or governed alternative sourcing outcome, and the complete set reconciles to the original demand without duplication, hidden value, or lost residual quantity.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when valid approved demand is identified as requiring more than one sourcing package, method, lot, timing window, supplier market, or outcome before supplier commitment.

## 4.2 End Boundary

The scenario ends when every approved split component has an attributable sourcing outcome or explicit residual, cancelled, returned, or deferred disposition and the total position reconciles to the source demand.

## 4.3 In Scope

- validating the source demand, approval, remaining unsourced quantity, value reference, ownership, and restrictions;
- recording the legitimate reason for splitting;
- testing the proposal against anti-fragmentation and aggregate-threshold rules;
- defining controlled components by line, quantity, lot, category, specification, location, delivery phase, funding source, method, or other approved dimension;
- preserving source-to-component and component-to-outcome lineage;
- determining sourcing method and authority using aggregate value where policy requires;
- approving the split plan before separate sourcing begins;
- creating and coordinating multiple method-specific Sourcing Events;
- governing component addition, withdrawal, correction, cancellation, recombination, or reassignment;
- preventing duplicate sourcing, overlapping quantity, double commitment, and orphaned demand;
- reconciling awarded, partially awarded, unawarded, deferred, cancelled, and residual positions;
- publishing traceable sourcing outcomes for downstream commitment.

## 4.4 Out of Scope

- creation, approval, amendment, withdrawal, or cancellation of J03-owned demand;
- consolidation of multiple approved demands, governed by L3-J-008;
- multi-supplier allocation within an evaluated award, governed by L3-J-010;
- detailed RFQ, RFP, tender, direct, rate-contract, emergency, or other method execution;
- supplier master and qualification lifecycle, owned by J02;
- Purchase Order, contract, commitment, payable, receipt, inventory, asset, or payment processing;
- algorithm, API, schema, workflow engine, or user-interface design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-J-008 Consolidated Approved Demand to Traceable Sourcing Outcome | Governs the inverse many-to-one sourcing pattern. |
| Related | L3-J-010 Evaluated Procurement Demand to Approved Multi-Supplier Award | Governs distribution among suppliers after evaluation rather than pre-sourcing division. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns split eligibility, component plan, aggregate-method compliance, sourcing-event traceability, reconciliation, residual treatment, recommendation, approval, and final outcome set. |

J03 remains authoritative for the original demand and approval. J01 creates sourcing components as controlled relationships to that demand, not new independent demand authority. Component outcomes cannot exceed or redefine the approved source.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Procurement Officer | Administers the governed sourcing activity and preserves the controlled sourcing record. | Performs sourcing administration. |
| Requesting Unit Representative | Supplies and confirms requirement and specification context without changing procurement ownership. | Provides business input. |
| Technical Evaluator | Evaluates technical compliance where material. | Evaluates; does not approve the sourcing outcome. |
| Commercial Evaluator | Evaluates commercial evidence where material. | Evaluates; does not approve the sourcing outcome. |
| Committee Member | Participates in configured collective review where required. | Participates within configured committee authority. |
| Procurement Recommender | Records the attributable supplier or outcome recommendation. | Recommends. |
| Procurement Approver | Approves, rejects or returns the sourcing outcome within established authority. | Decides. |
| Auditor / Reviewer | Reviews sourcing evidence and decision traceability without changing the outcome. | Reviews evidence. |
---

# 7. Preconditions and Entry Criteria

- An identifiable J03 demand has valid approval and a positive remaining unsourced quantity or scope.
- The demand has not already been fully sourced, committed, cancelled, withdrawn, or superseded.
- Lines, quantities, units, specifications or outcomes, delivery dates, locations, category, ownership, funding references, and value basis are available.
- A legitimate sourcing reason may require separate packages, methods, lots, locations, phases, supplier markets, or schedules.
- Applicable policy defines aggregation, threshold, split-purchase, competition, approval, and exception controls.
- The complete source demand and related procurements can be considered when determining method and authority.
- Component boundaries can be defined without overlapping quantity or changing the approved purpose.
- Required requester, technical, funding, and procurement confirmations can be obtained.

---

# 8. Trigger

Procurement determines that one approved demand cannot or should not be sourced through one undifferentiated sourcing package and proposes a governed split.

---

# 9. Main Business Flow

1. The Procurement Officer accepts the approved demand and records its J03 identity, lines, remaining quantities or scope, and approval references.
2. The officer validates demand status, purpose, specifications, categories, value reference, delivery, locations, funding context, and existing sourcing or commitment correlations.
3. The split rationale is recorded, such as distinct categories, specialist markets, incompatible specifications, geographical delivery, phased need, funding restrictions, risk separation, lot structure, capacity, or different legitimate sourcing methods.
4. Related demand, cumulative purchases, anticipated total value, and prior or parallel sourcing are reviewed for threshold and anti-fragmentation compliance.
5. If the split appears intended to avoid competition, publication, tender, committee, or approval requirements, it is rejected or escalated.
6. The officer defines proposed components with unique identities, source lines, quantities or scope, units, locations, dates, lots, method candidates, value references, and responsible actors.
7. Component quantities and scope are checked for overlap, gaps, invalid conversion, and total reconciliation to the source demand.
8. Requesting and specialist reviewers confirm that component boundaries preserve the approved purpose and acceptance obligations.
9. Applicable sourcing method and authority for each component are determined using aggregate value and related-demand rules where required, not merely component value.
10. A controlled split plan is created with rationale, membership version, allocation rules, sequence, dependencies, risk, residual handling, and approval route.
11. Required procurement, requester, technical, finance or funding, and competent-authority approvals are obtained before independent sourcing begins.
12. A method-specific Sourcing Event is created for each approved component and linked to the split plan and original demand.
13. Components proceed through their applicable RFQ, RFP, tender, direct, rate-contract, emergency, or other governed sourcing scenarios.
14. Supplier requests, submissions, evaluations, recommendations, and decisions remain attributable to the correct component.
15. A component change, withdrawal, cancellation, or failure is assessed for its effect on the source balance, other components, aggregate method, fairness, dependency, and timing.
16. Each completed component records awarded supplier or outcome, awarded quantity or scope, evaluated commercial reference, residual, conditions, and downstream correlation.
17. The Procurement Officer reconciles all component outcomes to the original demand and identifies awarded, partially awarded, unawarded, deferred, cancelled, removed, or still-unsourced positions.
18. The Procurement Recommender submits the complete component outcome set, reconciliation, residual treatment, exceptions, and risks for approval.
19. The Procurement Approver verifies source authority, split rationale, aggregate compliance, component outcomes, non-overlap, and reconciliation.
20. On approval, the split scenario reaches Completed and the component sourcing outcomes become available for downstream commitment.
21. J01 publishes traceable correlations without altering J03-owned approval or allowing the total committed position to exceed approved demand.

---

# 10. Alternate Business Flows

## 10.1 Split by Category or Specification

Distinct item, service, works, technology, or specification families are placed into separate components where supplier markets or evaluation requirements materially differ. Aggregate threshold obligations remain visible.

## 10.2 Split by Location or Campus

Separate delivery or execution locations may justify component sourcing when logistics, local availability, tax, acceptance, schedule, or authority differ. Institutional aggregation rules still apply.

## 10.3 Split by Phase or Required Date

Demand may be phased when requirements arise at materially different times or implementation dependencies require sequencing. Planning delay cannot be used as a false urgency basis.

## 10.4 Split by Funding or Grant Condition

Components may reflect distinct funds, schemes, sponsors, or grant rules where combined sourcing would breach conditions. The commercial outcome remains traceable without merging financial truth.

## 10.5 Split Across Sourcing Methods

Different components may use different methods only where each method is valid after applying aggregate and related-demand rules. A low-value direct route cannot be justified solely because a larger demand was divided.

## 10.6 Component Added or Revised Before Issue

A pre-issue component change requires renewed reconciliation, value, method, authority, dependency, and approval checks. Prior split-plan versions remain evidence.

## 10.7 Component Changed After Issue

Post-issue change follows the applicable method's amendment, fairness, extension, restart, cancellation, or re-sourcing controls and updates the overall reconciliation.

## 10.8 Components Recombined

If separate treatment is no longer beneficial or valid before commitment, components may be recombined under authority. Active sourcing effects are governed and earlier component history is preserved.

## 10.9 Partial Component Outcome

A component may be partially awarded. Awarded and residual positions are recorded separately and residual quantity may be re-sourced only through a new traceable relationship.

## 10.10 Component Failure or Cancellation

Failure of one component does not silently cancel or fulfil others. Dependency impact is assessed and the failed component receives a governed residual, re-sourcing, cancellation, or demand-correction disposition.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Invalid Source Demand

If the source demand is withdrawn, superseded, cancelled, or materially changed, affected components are suspended until J03 provides authoritative resolution.

## 11.2 Improper Fragmentation

If the split reduces competition, publication, oversight, or authority contrary to policy, sourcing is stopped. Components are recombined, reclassified, escalated, or redirected under the correct aggregate method.

## 11.3 Overlap or Quantity Imbalance

If components overlap, exceed approved quantity, omit required scope, or fail unit reconciliation, approval and downstream use are blocked until corrected.

## 11.4 Duplicate Sourcing or Commitment

If a source portion is already active in another event or committed outcome, the conflicting component is suspended until remaining demand and authoritative correlations are reconciled.

## 11.5 Component Method Error

If a component uses an invalid method or authority because only its isolated value was considered, it is corrected, cancelled, or restarted under the applicable aggregate control.

## 11.6 Dependency Failure

Where one component depends on another, delay, rejection, or cancellation triggers assessment of the remaining package sequence, compatibility, and institutional benefit.

## 11.7 Funding or Authority Conflict

Where a component conflicts with fund, grant, campus, unit, budget, or approval restrictions, that component is withheld or returned for authoritative resolution.

## 11.8 Reconciliation Failure

If completed component outcomes cannot reconcile to source lines, quantities, scope, and residuals, the overall split outcome cannot be approved.

## 11.9 Approval Interruption

An interrupted approval leaves the component outcome set unapproved. Individual method outcomes do not authorise downstream use beyond their own valid approval and the required overall controls.

## 11.10 Post-Completion Error

The completed split record is not destructively rewritten. Before downstream commitment, correction, cancellation, recombination, or re-sourcing applies; after commitment, downstream owners govern consequences.

---

# 12. Business Rules and Decision Constraints

- Only valid approved demand may be split for sourcing.
- Every split requires a legitimate recorded institutional or market reason.
- Splitting to avoid competition, publication, tender, committee, value, or approval controls is prohibited.
- Aggregate and related-demand value must govern method and authority where policy requires.
- Each component must have a unique identity and direct lineage to source demand lines, quantities, or scope.
- Component boundaries must be mutually exclusive unless an explicit governed overlap represents alternatives rather than additive sourcing.
- Total awarded and residual positions must reconcile to the valid source demand.
- Awarded or committed quantity must never exceed approved remaining demand.
- A source portion cannot be actively sourced or committed twice.
- Split components cannot change the approved purpose, specification, funding restriction, or acceptance obligation without authoritative demand change.
- Component method, value, schedule, location, lot, category, and owner must remain explicit.
- Post-issue component change requires method-specific fairness and amendment control.
- Cancellation or failure of one component does not imply fulfilment of the source demand.
- Residual demand must remain visible and traceable to any successor sourcing event.
- Recombined components retain prior identities and history.
- J01 does not create new demand approval or Finance-owned monetary truth.
- Approved component outcomes do not themselves create Purchase Orders, contracts, payables, receipts, accounting entries, inventory, assets, or payments.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Sourcing Event | `Draft` or the applicable existing J01 lifecycle position | Applies only the configured canonical sourcing stages required by this scenario. | `Completed` or the exact applicable terminal state: `Cancelled`, `No Response`, `No Suitable Bid` or `Re-Sourcing Required` | Procurement Planning & Sourcing (J01) | `L2-J01` Section 12.3 |
| Approved Procurement Demand | Source-defined approved position | Correlated to sourcing activity without changing demand state. | Unchanged source-owned position | Purchase Requisition & Approval (J03) | `L2-J03` |

Any scenario-specific consolidation, split, allocation, proposal or case record has no separately defined canonical lifecycle in J01. This scenario therefore does not create one.
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `SourcingEventCreated` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ApprovedSourcingOutcomeFinalised` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ReSourcingRequested` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
## 14.2 Cross-Domain Collaboration

| Domain / Capability | Collaboration | Boundary |
|---|---|---|
| J03 Purchase Requisition and Approval | Supplies approved demand, remaining quantity, scope, owner, timing, and budget-validation references. | J03 retains demand authority and lifecycle. |
| Requesting and specialist domains | Confirm category, specification, phasing, location, delivery, funding, and acceptance effects. | Each retains authoritative domain semantics. |
| Finance | Supplies monetary, funding, tax, historical, or budget references. | Finance retains monetary truth. |
| J02 Vendor and Supplier Management | Supplies supplier facts to component sourcing events. | J02 retains supplier ownership. |
| Method-specific J01 sourcing capabilities | Execute each component under the applicable sourcing method. | Each method retains its canonical controls. |
| J04 Purchase Order Management | Consumes approved component outcomes and residual-safe allocations. | J04 alone creates and manages Purchase Orders. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J01 roles are separated as institutional policy requires: Procurement Officer; Procurement Manager; Requesting Unit Representative; Technical Evaluator; Commercial Evaluator; Committee Member and Committee Chair / Convenor where used; Procurement Recommender; Procurement Approver; External Procurement Coordinator where applicable; and Auditor / Reviewer. A requester, evaluator, recommender, or conflicted participant cannot replace the Procurement Approver decision.

## 15.2 Required Evidence

- source demand identity, approval, lines, remaining quantities, scope, owner, specifications, locations, dates, funding and value references;
- split rationale and alternatives considered;
- aggregate-value, related-demand, prior-purchase, and anti-fragmentation assessment;
- component identities, lines, quantities, units, lots, locations, schedules, method, value, and owners;
- non-overlap, quantity conservation, dependency, risk, and residual rules;
- split-plan versions, confirmations, reviews, approvals, changes, and reasons;
- method-specific event references and sourcing evidence;
- component awarded, partial, failed, cancelled, deferred, and residual outcomes;
- complete reconciliation from source demand to components and outcomes;
- recommender, approver, authority, decisions, reasons, and timestamps.

## 15.3 Audit Obligations

The scenario must permit forward tracing from source demand to every component and outcome and reverse tracing from every outcome to its exact source portion. It must also demonstrate that splitting did not evade aggregate controls and that no quantity or scope was duplicated, lost, or falsely fulfilled.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the source demand was valid and approved at the applicable times;
- the split rationale is legitimate, documented, and authorised;
- aggregate value, related demand, competition, method, and authority were assessed correctly;
- every component is uniquely identified and mapped to source lines, quantities, or scope;
- component boundaries do not create unauthorised overlap or gaps;
- each component has an approved outcome or explicit residual, cancelled, deferred, or returned disposition;
- awarded and remaining quantities reconcile to the valid source demand;
- changes, recombinations, dependencies, and failed components have governed treatment;
- the complete outcome set receives valid approval;
- downstream commitment can consume component allocations without exceeding demand;

## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- all component sourcing events follow their applicable method governance;
- duplicate sourcing and double commitment are prevented;
- complete forward, reverse, and aggregate-control audit traceability is reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. `L2-J01` defines no separate split-plan or split-component lifecycle or event set. Governance must decide whether a supporting aggregate requires them; this L3 scenario uses only the canonical Sourcing Event lifecycle and events.
2. Institution-specific rules for aggregate-value periods, category grouping, campus or trust aggregation, repeat purchases, and permissible lotting cannot be universalised at L3.
3. The authoritative contract through which J03 exposes remaining unsourced quantity and consumes multiple sourcing-outcome facts requires confirmation.
4. The boundary between pre-sourcing split components, method-specific lots, partial awards, and L3-J-010 multi-supplier allocation requires final cross-scenario validation.
5. Overall completion semantics when some components remain deferred, disputed, cancelled, or under re-sourcing require operating-profile governance.

## 17.2 Assumptions

- Splitting creates controlled sourcing components, not new approved demands.
- Components may use different methods only when aggregate and related-demand rules still permit those methods.
- Method-specific scenarios retain execution authority and evidence obligations.
- Completed component outcomes are necessary but not sufficient for supplier commitment.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-009**
