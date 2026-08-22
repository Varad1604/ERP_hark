# Evaluated Procurement Demand to Approved Multi-Supplier Award

# 1. Scenario Metadata

**Scenario ID:** L3-J-010  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Procurement Planning & Sourcing  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** downstream J04 Purchase Order scenarios consuming approved supplier allocations; L3-J-008 and L3-J-009 where consolidated or split demand produces multi-supplier allocation  

---
# 2. Scenario Purpose

This scenario governs an award decision in which evaluated procurement demand is deliberately allocated across two or more eligible suppliers. It supports legitimate line, lot, quantity, location, capacity, risk-diversification, continuity, framework, ranked, primary-secondary, or other approved allocation models while preventing arbitrary distribution after bid visibility.

The scenario ensures that every supplier's awarded scope, quantity, ceiling, evaluated terms, rank, conditions, and source-demand relationship are approved and collectively reconcile to the evaluated requirement. It ends before Purchase Orders, contracts, call-offs, liabilities, receipts, or payments are created.

---

# 3. Business Outcome

Evaluated procurement demand results in an authorised multi-supplier award whose supplier-specific allocations, commercial basis, conditions, ceilings, residuals, and source-demand lineage are complete, non-overlapping, quantity-conserving, and auditable.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a governed sourcing evaluation has identified two or more eligible suppliers and the applicable sourcing conditions permit or require a multi-supplier award.

## 4.2 End Boundary

The scenario ends when the complete supplier-allocation recommendation receives valid approval and becomes an approved multi-supplier sourcing outcome, or when the proposal is returned, rejected, cancelled, or reduced to another authorised award structure.

## 4.3 In Scope

- validating demand, sourcing-event, supplier-response, and evaluation authority;
- confirming that multi-supplier allocation was allowed by the issued sourcing conditions or another valid pre-existing rule;
- identifying the approved allocation model and objective;
- allocating lines, lots, quantities, locations, delivery phases, capacities, ceilings, ranks, primary or secondary positions, and conditions;
- validating supplier eligibility, technical qualification, commercial position, capacity, delivery, risk, conflict, and dependency;
- preserving supplier-originated terms and any permitted equalisation, matching, negotiation, or common-rate treatment;
- applying pre-disclosed allocation, ranking, tie, capacity, minimum-share, maximum-share, reservation, preference, or fallback rules;
- preventing over-award, duplicate allocation, unsupported dilution, cherry-picking, and post-evaluation favouritism;
- recording awarded, partially awarded, unawarded, reserve, residual, and contingency positions;
- obtaining recommendation, exception, and final award approval;
- publishing supplier-specific allocations for downstream commitment.

## 4.4 Out of Scope

- demand or procurement-plan creation and approval;
- sourcing-method issue, response receipt, opening, and base evaluation, governed by applicable J01 scenarios;
- splitting approved demand into separate sourcing events before evaluation, governed by L3-J-009;
- supplier registration, qualification, suspension, or blacklisting, owned by J02;
- Purchase Order, contract, framework call-off, work order, amendment, or supplier commitment;
- budget commitment, accounting, payable, receipt, inspection, inventory, asset, or payment processing;
- allocation optimisation algorithm, API, schema, workflow, or user-interface design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-J-008 Consolidated Approved Demand to Traceable Sourcing Outcome | May supply consolidated line and owner allocations requiring multiple suppliers. |
| Related | L3-J-009 Split Approved Demand to Traceable Sourcing Outcomes | Distinguishes pre-sourcing component split from post-evaluation supplier allocation. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns multi-supplier eligibility, allocation model, supplier-specific award mapping, exception governance, recommendation, approval, residual treatment, and final award outcome. |

J01 consumes J03-approved demand and J02 supplier facts. Method-specific evaluation remains authoritative for supplier qualification and evaluated results. J01 may allocate only within the scope and rules established by the approved demand and sourcing event.

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

- Approved procurement demand or an authorised consolidated or split demand position is identifiable and has remaining awardable quantity or scope.
- The applicable Sourcing Event has completed the required evaluation stages and identified at least two eligible suppliers.
- Supplier responses, evaluated terms, qualifications, deviations, ranks, capacities, and validity are controlled and traceable.
- The issued sourcing conditions, approved evaluation plan, framework rule, or other authoritative pre-bid basis permits multi-supplier award and defines its allocation approach sufficiently.
- Allocation dimensions such as line, lot, quantity, location, phase, capacity, ceiling, rank, or primary-secondary status can be defined without overlap.
- Applicable approval, exception, conflict, competition, preference, and commercial rules are effective.
- J02 supplier status can be confirmed at the award decision time.
- No supplier-specific commitment has already consumed the same demand quantity or scope.

---

# 8. Trigger

The governed sourcing evaluation produces multiple eligible suppliers and the approved award rules require or permit distribution of the evaluated demand among them.

---

# 9. Main Business Flow

1. The Procurement Officer accepts the completed evaluation and records the authoritative demand, Sourcing Event, response, and evaluation references.
2. The coordinator validates remaining awardable lines, lots, quantities or scope, locations, delivery dates, estimated or evaluated value, and source-demand ownership.
3. The coordinator confirms that multi-supplier award was disclosed or otherwise validly authorised before proposal visibility and identifies the applicable allocation model.
4. Eligible suppliers, technical qualifications, commercial evaluations, ranks, capacities, delivery commitments, deviations, validity, and risk positions are assembled without changing the original evaluation.
5. J02 supplier eligibility, suspension, blacklisting, and relevant category status are confirmed for the award time.
6. Conflict-of-interest, related-party, concentration, dependency, continuity, and operational risks are reviewed where applicable.
7. The allocation objective is recorded, such as distinct lines or lots, capacity fulfilment, geographic coverage, phased delivery, risk diversification, continuity, reserved participation, primary-secondary arrangement, or ranked framework.
8. Pre-approved allocation rules are applied to determine supplier-specific lines, lots, quantities, percentages, ceilings, locations, phases, ranks, or roles.
9. Where award depends on matching a common or lowest evaluated rate, each supplier's eligibility, invitation to match, response, accepted terms, and affected allocation are preserved under the issued rules.
10. Supplier-originated commercial terms remain distinguishable from evaluated, negotiated, matched, or common award terms.
11. Every allocation is checked against supplier qualification, capacity, minimum or maximum share, delivery ability, lot restrictions, commercial validity, and prohibited deviations.
12. The combined allocation is checked for overlap, gaps, unsupported rounding, value inconsistency, duplicate sourcing, and quantity or scope conservation.
13. A supplier unable or unwilling to accept a permitted allocation is treated according to the pre-approved fallback, reallocation, reserve, re-sourcing, or residual rule.
14. The coordinator records unawarded demand, reserve suppliers, contingency positions, optional quantities, and residual sourcing needs without treating them as committed.
15. The Procurement Recommender prepares the complete award schedule showing each supplier, source demand, line or lot, quantity or share, location, phase, ceiling, rank, evaluated value reference, award terms, conditions, and justification.
16. Any departure from the issued allocation rule, rank order, common terms, capacity basis, or preference rule receives explicit exception evidence and authority, if such departure is legally permitted.
17. The Procurement Manager reviews fairness, competition, supplier eligibility, evaluation consistency, capacity, commercial treatment, risk, total allocation, and residuals.
18. The Procurement Approver reviews the exact complete schedule, exception authorities, aggregate value, delegated authority, and downstream consequences.
19. On approval, the recommendation becomes the approved multi-supplier award and the relevant Sourcing Event or award record reaches Completed.
20. Supplier-specific outcome facts are made available to J04 or another authorised commitment owner, each with ceilings and demand correlations.
21. J01 preserves the unallocated balance and prevents total downstream commitment from exceeding the approved award schedule or source demand.

---

# 10. Alternate Business Flows

## 10.1 Line or Lot Award

Different suppliers receive distinct lines or lots based on their evaluated eligibility and ranking. No line or lot is awarded twice unless the sourcing terms explicitly define shared or parallel supply.

## 10.2 Quantity or Percentage Split

One homogeneous requirement is divided by quantity or percentage among qualified suppliers according to pre-disclosed rank, capacity, risk, reservation, or share rules. Rounding and residual treatment remain explicit.

## 10.3 Location-Based Award

Suppliers receive approved campuses, regions, delivery points, or service areas where geographic capability and issued rules support separate allocation. Each location remains linked to source demand.

## 10.4 Phased or Time-Based Award

Suppliers receive different delivery phases or periods where capacity, continuity, risk, or implementation sequencing requires it. Phase allocation does not enlarge total approved demand.

## 10.5 Primary and Secondary Supplier

One supplier is primary and another is approved for fallback, overflow, contingency, or continuity. Secondary status does not create a guaranteed commitment unless expressly included in the approved terms.

## 10.6 Ranked Supplier Panel or Framework

Multiple suppliers may be approved with rank, category, ceiling, geographic scope, call-off rule, or rotation basis. Future call-offs remain governed separately and cannot exceed the approved framework authority.

## 10.7 Rate Matching or Common-Term Acceptance

Where disclosed and authorised, additional qualified suppliers may accept a reference rate or common commercial terms. Their original offers, match request, response, accepted terms, and allocation basis remain preserved.

## 10.8 Supplier Capacity Shortfall

If a higher-ranked supplier cannot meet the permitted quantity or schedule, remaining demand is allocated under the published fallback rule to another qualified supplier or retained as residual. Capacity is not invented after evaluation.

## 10.9 Supplier Declines Proposed Allocation

A supplier's decline is recorded. Reallocation follows the approved rule and does not permit arbitrary renegotiation or favouritism.

## 10.10 Partial Multi-Supplier Outcome

If qualified suppliers collectively cannot cover all demand, the approved award identifies fulfilled and residual positions. Residual demand may proceed to re-sourcing or authoritative change.

## 10.11 Recommendation Returned or Rejected

The approver may return the complete schedule for correction or reject it. Prior recommendation versions remain preserved and no supplier commitment is authorised.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Invalid Multi-Supplier Authority

If the issued sourcing conditions or pre-approved evaluation basis do not permit multiple awards, the proposal is withheld. A post-bid change cannot be invented merely because several suppliers are attractive.

## 11.2 Supplier Eligibility Change

If an intended awardee becomes suspended, blacklisted, ineligible, expired, or otherwise restricted, its allocation is suspended and the published fallback, reallocation, residual, or re-sourcing rule applies.

## 11.3 Allocation Exceeds Demand

If combined quantities, percentages, scope, or ceilings exceed valid demand or approved sourcing authority, approval and downstream use are blocked until corrected.

## 11.4 Overlap or Unallocated Gap

Conflicting line, lot, location, phase, or supplier mappings require correction. Any gap remains explicit residual demand and cannot be hidden through a completed status.

## 11.5 Unsupported Capacity or Risk Assumption

If capacity, delivery, resilience, or dependency evidence is missing or inconsistent, the affected allocation is returned for evaluation, supplier clarification where permitted, reallocation, or residual treatment.

## 11.6 Commercial Inconsistency

Unexplained differences in matched rates, discounts, charges, warranty, delivery, validity, or award conditions require correction and re-review. Original offers remain preserved.

## 11.7 Improper Rank or Rule Override

If the proposed allocation departs from published rank, share, lot, preference, or fallback rules without valid authority, the recommendation is withheld, corrected, or re-sourced.

## 11.8 Supplier Withdrawal or Refusal

The original evaluated result and refusal remain evidence. Reallocation follows the approved rule; otherwise affected demand remains residual.

## 11.9 Approval Interruption

An interrupted approval leaves the whole recommendation unapproved unless the governing model permits independently approved award components. No unapproved component may create commitment.

## 11.10 Post-Completion Error

An approved award is not destructively rewritten. Before downstream commitment, governed correction, cancellation, or successor recommendation applies. After commitment, J04 and other downstream owners govern their consequences.

---

# 12. Business Rules and Decision Constraints

- Multi-supplier award must originate from valid evaluated demand and an authorised sourcing event.
- The possibility and basis of multiple awards must be disclosed or otherwise validly established before supplier-response visibility.
- Allocation cannot be invented after evaluation to favour a supplier or dilute the evaluated winner without authority.
- Every awardee must be eligible and qualified for the exact allocated line, lot, quantity, location, phase, or role.
- Allocation must follow the approved rank, lot, share, capacity, ceiling, preference, reservation, rotation, fallback, or risk rule.
- Supplier-originated terms remain distinguishable from evaluated, matched, negotiated, or common award terms.
- Rate matching, negotiation, auction, or common-term acceptance is permitted only when the issued rules allow it.
- Total awarded quantity, scope, percentage, and ceiling cannot exceed valid demand or sourcing authority.
- Supplier allocations must not overlap unless parallel or shared supply is expressly authorised.
- Unawarded and contingency quantities remain visible and do not imply commitment.
- A reserve, panel, or secondary supplier has only the authority explicitly stated in the award.
- Capacity, delivery, risk, concentration, continuity, and dependency conclusions require evidence.
- A higher price or non-first-ranked allocation requires the justification and rule applicable to that allocation model.
- Preference or reservation rules must be applied consistently and remain attributable.
- Material approver changes require a successor recommendation rather than silent editing.
- All supplier allocations and residuals must reconcile to source demand and any consolidation or split structure.
- J01 does not create supplier commitment or Finance-owned monetary truth.
- The approved award does not itself create a Purchase Order, contract, payable, receipt, accounting entry, inventory, asset, or payment.

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
| `SplitAwardRecommended` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `AwardRecommendationSubmitted` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `AwardRecommendationApproved` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ApprovedSourcingOutcomeFinalised` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
## 14.2 Cross-Domain Collaboration

| Domain / Capability | Collaboration | Boundary |
|---|---|---|
| J03 Purchase Requisition and Approval | Supplies approved demand, source lines, quantities, owner, purpose, and budget-validation references. | J03 retains demand authority. |
| J02 Vendor and Supplier Management | Supplies supplier identity, eligibility, qualification, suspension, blacklisting, category, and risk facts. | J02 retains supplier ownership. |
| Method-specific J01 sourcing capability | Supplies issued allocation rules, responses, evaluation, ranks, capacities, terms, and valid suppliers. | Base evaluation remains authoritative. |
| Requesting and specialist domains | Supply delivery, location, technical, continuity, implementation, and acceptance context. | Each retains specialist semantics. |
| Finance | Supplies budget, monetary, tax, historical, or commercial references where required. | Finance retains monetary truth. |
| J04 Purchase Order Management or contracting capability | Consumes approved supplier-specific allocations and ceilings. | Downstream owner alone creates commitment. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J01 roles are separated as institutional policy requires: Procurement Officer; Procurement Manager; Requesting Unit Representative; Technical Evaluator; Commercial Evaluator; Committee Member and Committee Chair / Convenor where used; Procurement Recommender; Procurement Approver; External Procurement Coordinator where applicable; and Auditor / Reviewer. A requester, evaluator, recommender, or conflicted participant cannot replace the Procurement Approver decision.

## 15.2 Required Evidence

- approved demand and source-line or lot positions;
- Sourcing Event, issued multi-award rules, supplier responses, evaluation, rank, qualification, and validity;
- supplier eligibility snapshot, conflicts, risk, capacity, delivery, and dependency evidence;
- selected allocation model and objective;
- supplier-specific line, lot, quantity, percentage, location, phase, ceiling, rank, role, and conditions;
- original, evaluated, matched, negotiated, and approved commercial positions;
- share, capacity, preference, reservation, tie, fallback, rounding, and residual calculations;
- supplier acceptance or decline where required;
- exceptions, reasons, competent authority, and impact;
- complete reconciliation to source demand, consolidation, or split components;
- reviewer, recommender, approver, decisions, authority, and timestamps.

## 15.3 Audit Obligations

The scenario must support reconstruction from evaluated demand and issued allocation rules to every supplier-specific award and residual, and reverse tracing from each award to the exact response, evaluation, source demand, rule, reason, and authority. Original and successor allocations remain attributable.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- valid evaluated demand and multi-supplier authority are traceable;
- every intended awardee is eligible and qualified for its exact allocation;
- the allocation model and objective conform to issued or pre-approved rules;
- supplier-specific lines, lots, quantities, percentages, locations, phases, ceilings, ranks, roles, and conditions are explicit;
- original and evaluated, matched, negotiated, or common commercial terms remain distinguishable;
- capacity, delivery, risk, dependency, conflict, and commercial checks are complete;
- allocation follows approved rank, share, preference, reservation, fallback, and exception rules;
- combined awards do not overlap or exceed valid demand;
- unawarded, reserve, optional, contingency, and residual positions are explicit;
- the complete award schedule and exceptions receive valid approval;
- supplier-specific outcomes can be consumed downstream without creating commitment at J01;

## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- complete demand-to-award and award-to-evaluation traceability is reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. `L2-J01` defines no separate award-allocation lifecycle or event set. Governance must decide whether a supporting aggregate requires them; this L3 scenario uses only the canonical Sourcing Event lifecycle and events.
2. Institution-specific rules for quantity splits, minimum and maximum shares, ranked panels, primary-secondary arrangements, rate matching, reservations, preferences, and rotation cannot be universalised at L3.
3. The exact boundary between multi-supplier award, line or lot award within a tender, rate-contract panels, L3-J-009 split sourcing, and downstream multiple Purchase Orders requires final cross-scenario validation.
4. The authoritative business contract for supplier acceptance or decline of a proposed share before final award requires confirmation.
5. Residual, optional, reserve, and contingency quantity semantics and the point at which they consume demand authority require operating-profile governance.

## 17.2 Assumptions

- The evaluated sourcing basis permits more than one award and determines the allowed allocation logic.
- A multi-supplier award is one governed sourcing outcome containing multiple supplier allocations, not multiple newly approved demands.
- J04 may create separate commitments for approved allocations, but J01 does not create them.
- Reserve or secondary status does not imply guaranteed business unless expressly authorised.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-010**
