# Rate Contract Requirement to Approved Active Rate Contract

# 1. Scenario Metadata

**Scenario ID:** L3-J-011  
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
**Referenced By:** L3-J-012 Approved Demand to Rate Contract Sourcing Outcome; L3-J-013 Expiring Rate Contract to Approved Successor Arrangement; L3-J-014 Active Rate Contract to Governed Suspension or Closure  

---
# 2. Scenario Purpose

This scenario governs establishment of a rate contract for recurring or forecast institutional requirements through an authorised sourcing process. It ensures that supplier eligibility, covered catalogue or scope, rates, taxes and charges, validity, ceilings, locations, call-off rules, performance conditions, and approvals are controlled before the arrangement becomes available for demand sourcing.

Activation does not itself create a purchase commitment or guarantee consumption. Each subsequent demand and call-off remains separately governed.

---

# 3. Business Outcome

A fully evaluated and approved rate-contract arrangement becomes Active with traceable suppliers, covered items or services, rates, validity, ceilings, operating conditions, and downstream call-off authority.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an approved institutional requirement exists to establish a rate contract for recurring, standardised, or forecast procurement needs.

## 4.2 End Boundary

The scenario ends when the exact rate-contract version receives valid approval and reaches Active, or when establishment is rejected, cancelled, or directed to re-sourcing.

## 4.3 In Scope

- validating the requirement, forecast, categories, duration, locations, anticipated value, and policy basis;
- selecting and executing the applicable sourcing method;
- defining eligible suppliers, catalogue or scope, units, rates, taxes and charges, discounts, validity, ceilings, minimum or maximum commitments, delivery and service levels;
- defining single-supplier, multi-supplier, ranked, rotation, lowest-rate, choice, mini-competition, or other approved call-off rules;
- evaluating supplier capability, capacity, commercial position, risk, security, warranty, support, and performance conditions;
- controlling negotiation, rate matching, price-adjustment formula, indexation, renewal restriction, and extension authority where applicable;
- recording institutional, campus, unit, fund, category, and geographic applicability;
- approving and activating the controlled contract version;
- publishing availability without creating a Purchase Order or guaranteed consumption.

## 4.4 Out of Scope

- individual approved-demand call-off, governed by L3-J-012;
- successor arrangement, governed by L3-J-013;
- suspension or closure, governed by L3-J-014;
- supplier-master lifecycle, owned by J02;
- Purchase Order, contract execution mechanics, receipt, payable, accounting, or payment processing;
- API, schema, pricing engine, catalogue UI, or technical enforcement design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | L3-J-012 Approved Demand to Rate Contract Sourcing Outcome | Consumes the Active arrangement for approved demand. |
| Related | L3-J-013 Expiring Rate Contract to Approved Successor Arrangement | Preserves continuity through a governed successor. |
| Related | L3-J-014 Active Rate Contract to Governed Suspension or Closure | Governs loss or end of availability. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns the rate-contract requirement, sourcing, evaluation, commercial schedule, call-off governance, approval, version, activation, and availability outcome. |

J02 supplies supplier facts and Finance supplies monetary or tax references where required. J01 owns procurement meaning but does not create accounting truth or downstream commitments.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Procurement Officer | Administers the rate-contract sourcing or lifecycle activity and preserves its evidence. | Performs procurement administration. |
| Procurement Manager | Supervises the governed procurement activity. | Reviews and supervises. |
| Requesting Unit Representative | Supplies requirement and forecast context without acquiring procurement authority. | Provides business input. |
| Procurement Recommender | Records the attributable recommendation. | Recommends. |
| Procurement Approver | Approves, rejects or returns the governed outcome within established authority. | Decides. |
| Auditor / Reviewer | Reviews contract, decision and lifecycle evidence without modifying the record. | Reviews evidence. |
---

# 7. Preconditions and Entry Criteria

- A valid institutional requirement and forecast or recurring-demand basis exists.
- Categories, specifications or outcomes, units, anticipated volume, locations, duration, and service expectations are sufficiently defined.
- Applicable policy permits a rate contract and defines sourcing, competition, authority, duration, extension, and call-off controls.
- Aggregate anticipated value informs the sourcing method and approval authority.
- Supplier eligibility can be resolved through J02.
- Budget references may exist, but activation is not treated as budget commitment.
- No conflicting active contract covers the same scope unless overlap is explicitly authorised and bounded.
- Evaluation, multi-supplier allocation, price-adjustment, ceiling, and call-off rules are defined before supplier selection.

---

# 8. Trigger

Procurement accepts the approved requirement to establish a rate contract for recurring or forecast demand.

---

# 9. Main Business Flow

1. Procurement validates requirement authority, scope, forecast, categories, locations, duration, anticipated value, and policy.
2. The applicable sourcing method and approval path are determined using aggregate value and risk.
3. A controlled Sourcing Event is created and linked to the rate-contract requirement.
4. The sourcing package defines covered items or services, specifications, units, rate format, price basis, taxes and charges, delivery, warranty, service levels, validity, ceilings, and required evidence.
5. Call-off rules are defined before issue, including eligible demand, authorised units, supplier selection, ranking or rotation, mini-competition where applicable, quantity ceilings, and exceptions.
6. Extension, price adjustment, indexation, replacement, suspension, and closure conditions are stated where applicable.
7. Suppliers are invited or the requirement is published under the approved method and eligible responses are received and preserved.
8. Technical, commercial, capacity, risk, and service evaluations are completed under the issued criteria.
9. Original offered rates remain distinguishable from evaluated, matched, negotiated, or approved rates.
10. Where multiple suppliers are intended, supplier-specific catalogue, rank, location, ceiling, capacity, and call-off position are established under the approved rules.
11. The recommendation records suppliers, catalogue lines, units, approved rates, taxes and charges, validity period, ceilings, conditions, service levels, price-adjustment basis, and exceptions.
12. Supplier eligibility is rechecked at the award time and conflicts or restrictions are resolved.
13. The Procurement Manager reviews competition, value, evaluation, commercial schedule, overlap, authority, and downstream controls.
14. The Procurement Approver approves, rejects, or returns the exact controlled version.
15. On approval, the Rate Contract receives a unique identity and version and reaches Approved.
16. The effective date is reached only after required acceptance, securities, documents, and activation prerequisites are satisfied.
17. The Rate Contract reaches Active and becomes discoverable for eligible demand.
18. Activation publishes authoritative availability facts without creating any Purchase Order, guaranteed volume, liability, or payment.

---

# 10. Alternate Business Flows

## 10.1 Single-Supplier Contract

One evaluated supplier covers the approved catalogue and applicability. Call-offs remain bounded by validity, ceiling, eligibility, and demand approval.

## 10.2 Multi-Supplier Contract

Several suppliers are activated with line, rank, location, ceiling, rotation, mini-competition, or choice rules. Each supplier's exact authority is explicit.

## 10.3 Rate Matching or Negotiation

Where issued rules permit, qualified suppliers may match or negotiate approved terms. Original and final offers, authority, participants, and allocation effects remain preserved.

## 10.4 Indexed or Adjustable Rate

The contract may include a defined index, formula, review date, evidence source, cap, floor, and approval rule. Future adjustment does not rewrite the original rate.

## 10.5 Phased Activation

Different suppliers, locations, or catalogue sections may activate on different dates only when the controlled version records each prerequisite and effective period.

## 10.6 Establishment Rejected or Re-Sourced

If competition, evaluation, commercial reasonableness, or authority is insufficient, the recommendation is rejected, cancelled, or returned to sourcing without creating availability.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Invalid Requirement or Method

Missing authority, improper aggregation, invalid method, or artificial fragmentation suspends establishment until corrected or re-sourced.

## 11.2 Supplier Eligibility Change

An intended supplier becoming ineligible before activation is withheld. Reallocation, re-sourcing, or reduced activation follows the issued rules and authority.

## 11.3 Commercial Schedule Error

Ambiguous units, duplicated lines, inconsistent rates, taxes, ceilings, validity, or conditions require correction and re-approval. Original versions remain evidence.

## 11.4 Missing Activation Prerequisite

Absent acceptance, security, insurance, document, licence, or other required evidence keeps the contract Approved but not Active, or triggers rejection under policy.

## 11.5 Overlapping Arrangement

Uncontrolled overlap with another active contract is blocked. Authorised overlap must define distinct scope, geography, period, supplier use, and precedence.

## 11.6 Approval Interruption

An interrupted decision leaves the arrangement unapproved and unavailable for call-off.

## 11.7 Post-Activation Error

The Active version is not destructively rewritten. Governed amendment, suspension, successor, or closure applies, preserving historical availability and call-offs.

---

# 12. Business Rules and Decision Constraints

- Rate-contract establishment requires authorised institutional need and an applicable sourcing method.
- Aggregate anticipated value governs method and authority where required.
- Rates, units, taxes, charges, validity, ceilings, catalogue, applicability, and call-off rules must be explicit before activation.
- Supplier-originated and evaluated or approved commercial facts must remain distinguishable.
- Multi-supplier rank, rotation, choice, mini-competition, capacity, and fallback rules must be approved before use.
- An Active rate contract does not guarantee volume unless the controlled terms expressly create a minimum commitment and separate authority supports it.
- Call-offs cannot exceed supplier, line, period, institutional, or aggregate ceilings.
- Demand approval is required for every downstream use.
- Price adjustment or extension must follow the original authorised basis or a separately governed successor or amendment.
- Expiry cannot be avoided by backdating a call-off or extending validity silently.
- Suspended, closed, expired, or ineligible supplier positions cannot authorise new call-offs.
- Historical versions, rates, supplier positions, and call-offs cannot be overwritten.
- Activation does not create a Purchase Order, payable, receipt, accounting entry, inventory, asset, or payment.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Rate Contract | `Draft` | Proceeds through the exact J01 lifecycle positions `Review` and `Approved` when applicable. | `Active` when the approved arrangement becomes effective | Procurement Planning & Sourcing (J01) | `L2-J01` Section 27.3 |
| Approved Procurement Demand | Source-defined approved position | Supplies the governed requirement without changing demand state. | Unchanged source-owned position | Purchase Requisition & Approval (J03) | `L2-J03` |
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `RateContractCreated` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `RateContractApproved` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `RateContractActivated` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| J03 | Supplies approved-demand and forecast references. | Retains demand ownership. |
| J02 | Supplies supplier identity, eligibility, qualification, suspension, and risk facts. | Retains supplier ownership. |
| Finance | Supplies tax, monetary, budget, index, and historical-price references. | Retains monetary truth. |
| Requesting domains | Supply specification, forecast, location, service, and acceptance context. | Retain specialist semantics. |
| J04 | Consumes approved rate-contract sourcing outcomes for Purchase Order creation. | Alone creates commitment. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J01 roles are separated as institutional policy requires: Procurement Officer; Procurement Manager; Requesting Unit Representative; Technical Evaluator; Commercial Evaluator; Committee Member and Committee Chair / Convenor where used; Procurement Recommender; Procurement Approver; External Procurement Coordinator where applicable; and Auditor / Reviewer. A requester, evaluator, recommender, or conflicted participant cannot replace the Procurement Approver decision.

## 15.2 Required Evidence

- requirement, forecast, category, scope, value, locations, and policy;
- sourcing method, issued conditions, supplier responses, evaluation, competition, and commercial reasonableness;
- supplier eligibility and conflict evidence;
- catalogue, units, rates, taxes, charges, discounts, validity, ceilings, service levels, locations, and call-off rules;
- original, evaluated, negotiated, matched, and approved terms;
- securities, acceptance, licences, insurance, and activation prerequisites where applicable;
- recommendation, approval, effective date, actor, authority, reason, and version.

## 15.3 Audit Obligations

The scenario must reconstruct requirement, sourcing, supplier evaluation, commercial schedule, approval, prerequisites, and activation. Every active line and supplier must trace to the issued rule, response, evaluation, and authority.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the requirement, aggregate value, method, and authority are valid;
- supplier eligibility and evaluation are complete;
- covered catalogue or scope, units, rates, taxes, charges, validity, ceilings, applicability, and service conditions are controlled;
- call-off, multi-supplier, fallback, adjustment, extension, suspension, and closure rules are defined where applicable;
- original and approved commercial facts remain distinguishable;
- exact supplier positions and conditions receive approval;
- activation prerequisites and effective date are satisfied;
- the version reaches Active and is available only within its scope;
- no purchase commitment or guaranteed volume is inferred merely from activation;

## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- complete audit evidence is reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

2. Institution-specific limits for duration, extension, ceilings, minimum commitment, rate matching, rotation, and mini-competition cannot be universalised at L3.
3. The boundary between procurement rate contract, legal contract execution, framework agreement, empanelment, and J04 call-off requires final validation.
4. Price-adjustment, tax-change, and indexation authority require operating-profile configuration.

## 17.2 Assumptions

- Activation makes the arrangement eligible for use but does not itself consume budget or demand.
- Supplier acceptance and required evidence precede Active status where policy requires.
- Historical versions remain immutable business evidence.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-011**
