# Consolidated Approved Demand to Traceable Sourcing Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-J-008  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Procurement Planning & Sourcing  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** L3-J-004 through L3-J-007 sourcing-method scenarios; downstream J04 Purchase Order scenarios consuming approved sourcing outcomes  

---
# 2. Scenario Purpose

This scenario governs the consolidation of multiple approved procurement demands into one traceable sourcing exercise when their requirements are compatible and consolidation produces a legitimate institutional benefit. It prevents aggregation from obscuring the authority, ownership, quantity, funding reference, delivery need, or outcome of any contributing demand.

The scenario preserves a many-to-one lineage from approved demand lines to the consolidated package and then from the approved sourcing outcome back to every included line. It does not approve demand, select a sourcing method contrary to policy, or create supplier commitment.

---

# 3. Business Outcome

Eligible approved demand lines are consolidated under authorised rules, sourced through the applicable governed method, and resolved into an approved sourcing outcome whose allocation, residuals, exceptions, and evidence remain traceable to every contributing demand.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when two or more valid approved demand records or lines are identified as candidates for consolidation before supplier commitment.

## 4.2 End Boundary

The scenario ends when the consolidated sourcing exercise has an approved sourcing outcome with complete source-demand allocation and residual treatment, or when consolidation is rejected, dissolved, cancelled, or redirected through an authorised disposition.

## 4.3 In Scope

- identifying candidate approved demands and lines;
- validating approval, status, remaining quantity, ownership, specification, category, timing, location, funding, and other consolidation attributes;
- assessing compatibility and institutional benefit;
- aggregating anticipated value for method and authority determination;
- preventing duplication, over-sourcing, double commitment, or hidden threshold avoidance;
- recording consolidation membership, allocation basis, responsible coordinator, and approvals;
- creating a consolidated sourcing package without altering the underlying demand truth;
- governing inclusion, exclusion, withdrawal, quantity correction, cancellation, or successor demand before sourcing closure;
- executing or handing off to the applicable RFQ, RFP, tender, direct, rate-contract, or other authorised sourcing method;
- mapping supplier responses, evaluation, recommendation, and final outcome to source demand lines;
- allocating awarded and unawarded quantities, values, lots, delivery positions, and residual demand;
- publishing the traceable outcome for downstream commitment.

## 4.4 Out of Scope

- creating, approving, amending, withdrawing, or cancelling the authoritative demand in J03;
- supplier registration or qualification, owned by J02;
- detailed sourcing-method behaviour already governed by L3-J-004 through L3-J-007 and other method-specific scenarios;
- splitting one approved demand into multiple sourcing outcomes, governed by L3-J-009;
- multi-supplier award evaluation, governed by L3-J-010 where applicable;
- Purchase Order, contract, budget commitment, payable, receipt, inventory, asset, or payment processing;
- database, matching algorithm, allocation engine, API, technical event structure, or user-interface design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-J-009 Split Approved Demand to Traceable Sourcing Outcomes | Governs the inverse one-to-many sourcing pattern. |
| Related | L3-J-010 Evaluated Procurement Demand to Approved Multi-Supplier Award | Governs allocation to multiple suppliers after evaluation. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns consolidation eligibility, consolidated package, sourcing traceability, allocation, residual treatment, recommendation, approval, and final sourcing outcome. |

J03 remains authoritative for each contributing demand and its approval state. J01 records sourcing membership and outcome correlation but must not replace several approved demands with a new unapproved business demand or silently change their meaning.

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

- At least two identifiable J03 demands or lines have valid approval and remaining unsourced quantity.
- Each candidate demand is active for sourcing and has not already been fully committed, cancelled, superseded, or included in an incompatible active event.
- Requirement descriptions, specifications or outcomes, units, quantities, delivery expectations, locations, funding references, and requesting owners are available.
- Applicable policy permits consolidation and defines value aggregation, method, approval, and exception requirements.
- Consolidation does not invalidate demand approvals, budget references, funding restrictions, grant conditions, campus or unit authority, or delivery needs.
- Candidate requirements are sufficiently compatible to be issued, evaluated, and awarded under a coherent sourcing package or governed lots.
- Responsible requesting units can confirm consolidation participation where required.
- No duplicate sourcing, double counting, or conflicting consolidation membership exists.

---

# 8. Trigger

Procurement identifies compatible approved demand records that may be sourced together for economy, competition, standardisation, administrative efficiency, coordinated delivery, or another legitimate institutional benefit.

---

# 9. Main Business Flow

1. The Procurement Officer identifies candidate approved demands and records their J03 identities and line references.
2. For each candidate line, the officer validates approval state, remaining unsourced quantity, unit, specification or outcome, required date, location, requesting owner, value reference, funding context, and existing sourcing or commitment correlations.
3. Duplicate, withdrawn, superseded, fully sourced, already committed, or otherwise ineligible lines are excluded with reason.
4. Candidate lines are compared for category, specification compatibility, acceptable alternatives, unit basis, evaluation requirements, delivery windows, locations, warranty or service needs, funding restrictions, and institutional policy.
5. Requesting units and specialist reviewers confirm whether requirements may be sourced together without changing authorised need or acceptance obligations.
6. Procurement records the institutional consolidation benefit, material risks, proposed lots, allocation basis, coordinator, and affected demand owners.
7. Anticipated values are aggregated as required. The resulting sourcing method, publication or competition duty, approval authority, and exception rules are determined from the consolidated position.
8. Any evidence of artificial fragmentation, threshold avoidance, incompatible funding, conflicting specifications, or unacceptable schedule impact prevents ordinary consolidation until resolved.
9. A controlled consolidation record is created with unique identity, membership version, included demand lines, quantities, exclusions, source owners, and rationale.
10. Required requesting-unit confirmations, technical review, procurement review, and consolidation approval are obtained.
11. The consolidated sourcing package is prepared with line, lot, quantity, delivery, location, specification, and allocation identifiers that retain source-demand lineage.
12. The applicable sourcing method is executed under its governing scenario. Supplier communications do not expose internal funding or ownership information beyond what the sourcing process legitimately requires.
13. Supplier submissions are evaluated against the consolidated package and each response remains attributable to applicable lines or lots.
14. If a source demand changes, is withdrawn, lapses, or becomes invalid before closure, the effect on package fairness, quantity, value, method, competition, and supplier responses is assessed before amendment or removal.
15. The evaluation establishes eligible supplier outcomes by line, lot, quantity, delivery position, or other approved allocation dimension.
16. The Procurement Recommender records the supplier recommendation, evaluated commercial reference, source-demand allocation, unawarded or residual quantity, conditions, risks, and exceptions.
17. For each contributing demand line, the recommendation shows included quantity, awarded quantity, remaining quantity, supplier or lot allocation, and applicable outcome reference.
18. The Procurement Approver reviews source authority, consolidation eligibility, aggregated method, evaluation, allocation, residuals, and delegated authority.
19. On approval, the consolidated Sourcing Event reaches Completed and the exact sourcing outcome becomes available for downstream commitment.
20. Each source demand receives a traceable sourcing-outcome correlation without J01 altering J03-owned approval or lifecycle truth.

---

# 10. Alternate Business Flows

## 10.1 Consolidation by Common Item or Service

Demands for the same or technically compatible item or service are combined into common lines. Unit conversions, specification equivalence, delivery locations, and ownership remain explicit.

## 10.2 Consolidation Through Lots

Where demands share one sourcing event but require distinguishable specifications, locations, schedules, or evaluation, governed lots preserve competition and allocation. Each lot retains source-demand membership.

## 10.3 Cross-Campus or Cross-Unit Consolidation

Demands from multiple campuses or units may be combined where authority, funding, delivery, tax or commercial context, and acceptance responsibilities permit. Consolidation does not transfer ownership between units.

## 10.4 Demand Added Before Issue

A newly approved compatible demand may join before supplier issue if membership, aggregate value, method, authority, package, timetable, and approvals are revalidated. Earlier membership versions remain evidence.

## 10.5 Demand Added After Issue

Post-issue addition is permitted only through a governed amendment that preserves fairness and provides adequate supplier response time. If material, the event is reissued, restarted, or cancelled rather than silently expanded.

## 10.6 Demand Withdrawal or Quantity Reduction

The authoritative J03 change is consumed and its effect on value, competition, method, supplier bids, and allocation is assessed. Removal or reduction is versioned and never erases original membership.

## 10.7 Partial Consolidation

Only compatible lines or quantities are consolidated. Remaining demand stays outside the event with an explicit unsourced or separately sourced position.

## 10.8 Consolidation Rejected or Dissolved

If compatibility, timing, funding, authority, fairness, or benefit is insufficient, the proposal is rejected or the consolidation is dissolved. Source demands remain separately available for authorised sourcing.

## 10.9 Multi-Supplier Outcome

Where the evaluated result allocates the consolidated requirement across suppliers, L3-J-010 governs award distribution while this scenario retains source-demand-to-award traceability.

## 10.10 No Suitable or Incomplete Outcome

Unawarded lots or quantities are recorded as residuals and may proceed to re-sourcing, alternate method, cancellation, or authoritative demand change. They are not treated as fulfilled.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Invalid Source Demand

If any included demand lacks valid approval or becomes withdrawn, superseded, or cancelled, its membership is suspended and the consolidation impact is assessed. J01 does not repair J03 demand.

## 11.2 Duplicate Membership or Double Sourcing

If a demand line is already included in another active sourcing event or committed outcome, processing is withheld until the authoritative relationship and remaining quantity are reconciled.

## 11.3 Incompatible Requirements

Conflicting specifications, evaluation methods, delivery dates, locations, funding conditions, or acceptance obligations require exclusion, lotting, separate sourcing, or authoritative demand correction.

## 11.4 Aggregation or Method Error

If consolidated value changes the required method or authority, the package is corrected before continuation. A lower-control route cannot be retained using pre-consolidation values.

## 11.5 Post-Issue Membership Change

A material addition, withdrawal, quantity change, or specification change triggers fairness and method assessment. Amendment, extension, restart, cancellation, or separate sourcing is chosen under authority.

## 11.6 Allocation Imbalance

If recommended quantities exceed source demand, omit included demand, violate lot rules, or cannot be attributed, approval is withheld until allocation is corrected and re-reviewed.

## 11.7 Funding or Authority Conflict

Where one source demand's funding, campus, approval, or grant restriction conflicts with the proposed award, that portion is withheld, separated, or returned for authoritative resolution.

## 11.8 Approval Interruption

An interrupted approval leaves the consolidated recommendation unapproved. No contributing demand may be treated as sourced until valid approval exists.

## 11.9 Post-Completion Error

A completed outcome is not destructively rewritten. Before downstream commitment, governed correction, cancellation, reallocation, or re-sourcing applies. After commitment, downstream owners govern their consequences.

---

# 12. Business Rules and Decision Constraints

- Only valid approved demand may participate in consolidation.
- Consolidation must have a recorded institutional benefit and must not change the authorised purpose of any demand.
- Every included source demand and line must remain independently identifiable.
- Consolidation membership, quantity, ownership, funding context, and versions cannot be overwritten.
- Requirements must be compatible enough for fair common issue and evaluation, or separated into governed lots.
- Aggregate anticipated value determines applicable method and authority where policy requires.
- Consolidation must not be used to bypass unit, campus, funding, grant, category, competition, or approval controls.
- Conversely, approved demands must not remain artificially fragmented to avoid an aggregate threshold or competitive method.
- A demand line cannot be actively sourced or committed twice for the same unsourced quantity.
- Unit conversions, equivalent specifications, substitutions, and quantity normalisation require explicit rules and evidence.
- Post-issue membership or quantity change requires fairness, method, value, and authority reassessment.
- Supplier responses and evaluations must remain traceable to the line or lot they address.
- Awarded quantity cannot exceed the valid remaining quantity of contributing demand.
- Every source demand line must end with an awarded, partially awarded, unawarded, removed, cancelled, or redirected position.
- Residual demand must remain visible and cannot be marked fulfilled merely because the consolidated event completed.
- Allocation across campuses, units, funds, locations, lots, or suppliers requires explicit approved mapping.
- J01 does not modify J03 demand approval or create financial truth.
- The approved sourcing outcome does not itself create a Purchase Order, contract, payable, receipt, accounting entry, inventory, asset, or payment.

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
| `ProcurementDemandConsolidated` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `SourcingEventCreated` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ApprovedSourcingOutcomeFinalised` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
## 14.2 Cross-Domain Collaboration

| Domain / Capability | Collaboration | Boundary |
|---|---|---|
| J03 Purchase Requisition and Approval | Supplies approved demand, line, quantity, owner, purpose, timing, and budget-validation references. | J03 retains demand authority and lifecycle. |
| Requesting units or specialist domains | Confirm compatibility, acceptable standardisation, delivery, location, and acceptance obligations. | Each unit retains its own business ownership. |
| Finance | Supplies funding, monetary, tax, historical, or budget references where required. | Finance retains monetary truth. |
| J02 Vendor and Supplier Management | Supplies supplier eligibility to the method-specific event. | J02 retains supplier ownership. |
| Method-specific J01 sourcing capability | Executes RFQ, RFP, tender, direct, rate-contract, or other authorised sourcing. | Method scenario retains its controls. |
| J04 Purchase Order Management | Consumes approved outcome allocations for downstream commitment. | J04 alone creates and manages Purchase Orders. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J01 roles are separated as institutional policy requires: Procurement Officer; Procurement Manager; Requesting Unit Representative; Technical Evaluator; Commercial Evaluator; Committee Member and Committee Chair / Convenor where used; Procurement Recommender; Procurement Approver; External Procurement Coordinator where applicable; and Auditor / Reviewer. A requester, evaluator, recommender, or conflicted participant cannot replace the Procurement Approver decision.

## 15.2 Required Evidence

- every source demand and line identity, approval state, owner, quantity, unit, specification, delivery, location, funding or budget reference, and remaining position;
- compatibility assessment and consolidation benefit;
- exclusions, duplicates, conflicting events, and split or aggregation review;
- aggregate value, selected method, policy basis, and approval authority;
- consolidation membership versions, lots, allocation rules, confirmations, and approvals;
- post-issue additions, removals, withdrawals, amendments, and fairness decisions;
- method-specific sourcing evidence and supplier-response linkage;
- recommendation by source demand, line, lot, supplier, quantity, value reference, and delivery position;
- awarded, partially awarded, residual, removed, cancelled, or redirected outcome for every source line;
- reviewers, recommender, approver, decisions, reasons, authority, and timestamps.

## 15.3 Audit Obligations

The scenario must support reconstruction in both directions: from every approved demand line to its consolidated package and outcome, and from every award component back to its source demands. Membership, changes, exclusions, evaluations, allocations, residuals, actors, timestamps, reasons, and authority remain attributable.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- all contributing demands were valid and approved at the applicable decision time;
- compatible requirements, consolidation benefit, source owners, and timing are evidenced;
- aggregate value, method, competition, and authority were determined from the consolidated position;
- duplicate sourcing, double commitment, artificial fragmentation, and conflicting membership were excluded;
- a controlled membership and sourcing-package version exists;
- method-specific sourcing completed under its applicable governance;
- every supplier response and evaluated outcome can be mapped to relevant lines or lots;
- every source demand line has an approved awarded, partially awarded, residual, removed, cancelled, or redirected position;
- allocated quantities do not exceed valid remaining demand;
- post-issue membership and quantity changes have authorised treatment;
- the exact consolidated recommendation and allocation received valid approval;
- downstream commitment can consume the allocations without J01 creating commitment;

## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- complete forward and reverse audit traceability is reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. `L2-J01` defines no separate consolidation-membership lifecycle or event set. Governance must decide whether a supporting aggregate requires them; this L3 scenario uses only the canonical Sourcing Event lifecycle and events.
2. The authoritative rules for aggregation windows, category grouping, cross-campus value, grant or fund combination, and repeated procurement remain institution-specific.
3. The exact business contract by which J03 exposes remaining unsourced quantity and consumes sourcing-allocation facts requires confirmation before locking.
4. The boundary between consolidation lots, split sourcing under L3-J-009, and multi-supplier award under L3-J-010 requires final cross-scenario validation.
5. Post-issue source-demand withdrawal, expiry, or material change requires operating-profile rules for amendment, reissue, cancellation, and residual handling.

## 17.2 Assumptions

- Consolidation may include multiple demands, lines, units, or campuses only when their authority and requirements remain independently traceable.
- Consolidation does not create a new demand approval; it creates a sourcing relationship among existing approved demands.
- Method-specific sourcing remains governed by the applicable canonical scenario.
- A completed consolidated outcome is necessary but not sufficient for supplier commitment.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-008**
