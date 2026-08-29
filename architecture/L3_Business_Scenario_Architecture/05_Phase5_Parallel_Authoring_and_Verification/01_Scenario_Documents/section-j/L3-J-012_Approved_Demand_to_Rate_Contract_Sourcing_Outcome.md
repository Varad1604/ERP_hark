# Approved Demand to Rate-Contract Sourcing Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-J-012  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Procurement Planning & Sourcing  
**Participating Domains:** J  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** downstream J04 Purchase Order scenarios consuming the approved call-off sourcing outcome  

---
# 2. Scenario Purpose

This scenario governs use of an Active rate contract to satisfy approved procurement demand. It ensures exact scope, supplier, rate, validity, ceiling, quantity, location, eligibility, call-off rule, and residual balance are validated at the decision time rather than assuming that contract existence automatically authorises purchase.

---

# 3. Business Outcome

Eligible approved demand is mapped to an Active rate contract and results in an authorised supplier-specific sourcing outcome within the contract's scope, rate, validity, ceiling, and call-off rules.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when valid approved demand is proposed for sourcing through an Active rate contract.

## 4.2 End Boundary

The scenario ends when a controlled call-off sourcing recommendation is approved, or when the demand is rejected, returned, deferred, or redirected because the rate contract cannot validly cover it.

## 4.3 In Scope

- validating demand approval, remaining quantity, specification, location, required date, and budget reference;
- identifying applicable Active rate-contract versions and lines;
- validating supplier status, rate, taxes and charges, validity, ceiling, balance, applicability, service level, and call-off rule;
- applying rank, rotation, mini-competition, choice, primary-secondary, or allocation rules;
- resolving multiple applicable contracts or suppliers under approved precedence;
- recording supplier, line, quantity, rate reference, delivery, conditions, residual, and approval;
- reserving or recognising sourcing utilisation at the business level without creating a Purchase Order.

## 4.4 Out of Scope

- rate-contract establishment, successor, suspension, or closure;
- demand creation or amendment;
- supplier-master lifecycle;
- Purchase Order, receipt, payable, accounting, and payment;
- technical balance reservation, locking, API, schema, or UI design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-011 Rate Contract Requirement to Approved Active Rate Contract | Supplies the Active arrangement. |
| Related | L3-J-013 Expiring Rate Contract to Approved Successor Arrangement | May redirect demand during transition. |
| Related | L3-J-014 Active Rate Contract to Governed Suspension or Closure | May block new sourcing. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns contract matching, eligibility, call-off rule, supplier selection, rate and ceiling validation, recommendation, approval, and sourcing outcome. |

J03 retains demand authority, J02 retains supplier truth, Finance retains monetary truth, and J04 alone creates Purchase Orders.

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

- J03 demand is approved and has remaining unsourced quantity or scope.
- At least one Rate Contract version is Active at the proposed sourcing date.
- Contract catalogue, suppliers, rates, validity, applicability, ceilings, and call-off rules are available.
- Supplier eligibility can be confirmed through J02.
- Demand specification, unit, location, delivery, and conditions can be matched without changing demand.
- No duplicate active sourcing or commitment consumes the same quantity.

---

# 8. Trigger

Approved demand is submitted to J01 and an Active rate contract is identified as a possible authorised sourcing route.

---

# 9. Main Business Flow

1. Procurement validates demand identity, approval, remaining quantity, scope, specification, unit, location, required date, and budget reference.
2. Applicable Active rate contracts are identified by category, item or service, institution, location, unit, and validity.
3. The exact controlled contract version and catalogue line are selected.
4. Specification, unit, minimum or maximum quantity, service level, delivery, warranty, and location fit are validated.
5. Supplier eligibility and contract position are checked at the proposed sourcing time.
6. Approved rate, taxes, charges, discounts, indexation or adjustment, validity, and commercial conditions are determined without overwriting original contract terms.
7. Contract, supplier, line, period, and aggregate ceiling and available business balance are checked.
8. The applicable call-off rule is applied: single supplier, rank, rotation, allocation, choice, primary-secondary, mini-competition, or another approved rule.
9. Where mini-competition is required, eligible contract suppliers are invited and evaluated under the contract's controlled rules.
10. The recommended supplier, contract line, quantity, location, delivery, rate reference, evaluated value, conditions, and residual demand are recorded.
11. Contract utilisation impact and any optional or unawarded quantity are made explicit.
12. Procurement reviews demand fit, supplier status, rate, ceiling, call-off rule, budget reference, and authority.
13. The approver approves, rejects, returns, or redirects the exact recommendation.
14. On approval, a rate-contract sourcing outcome is published for J04 without creating a Purchase Order or supplier commitment.

---

# 10. Alternate Business Flows

## 10.1 Single Applicable Supplier

The only authorised supplier is selected after all demand, status, validity, rate, and ceiling checks pass.

## 10.2 Multiple Suppliers

Rank, rotation, allocation, location, capacity, choice, or mini-competition rules determine selection. Familiarity or requester preference cannot override them.

## 10.3 Partial Contract Coverage

Only covered lines or quantities proceed through the contract. Remaining demand is explicitly redirected to another sourcing route.

## 10.4 Price Adjustment

An approved formula or indexed adjustment produces a dated successor rate reference while preserving the base rate, evidence, calculation basis, and authority.

## 10.5 Near-Expiry Demand

Demand may proceed only if the call-off and expected delivery or performance timing comply with contract conditions. Backdating is prohibited.

## 10.6 Ceiling Insufficient

Covered quantity is limited to valid balance; remaining demand is redirected, deferred, or addressed through a governed successor or amendment.

## 10.7 No Applicable Contract

The demand is redirected to the appropriate sourcing method and is not forced into an incompatible catalogue line.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Demand or Contract Invalid

Invalid demand, inactive contract, expired version, suspended supplier, or closed line blocks sourcing.

## 11.2 Specification or Unit Mismatch

Unapproved substitution, conversion, or scope mismatch requires authoritative correction or another sourcing route.

## 11.3 Ceiling or Balance Conflict

If quantity or value would exceed a ceiling, processing is withheld until reduced, redirected, or governed through another arrangement.

## 11.4 Concurrent Utilisation Conflict

If another sourcing or commitment may consume the same balance or demand, approval is withheld pending reconciliation.

## 11.5 Incorrect Supplier Selection

Rank, rotation, location, allocation, or mini-competition errors require correction and re-review.

## 11.6 Post-Approval Change

An approved outcome is not silently edited. Material changes require a successor recommendation or cancellation before downstream commitment.

---

# 12. Business Rules and Decision Constraints

- Only approved demand may consume rate-contract sourcing authority.
- Only an Active contract version and eligible supplier can support a new outcome.
- Demand must match covered scope, specification, unit, location, timing, and conditions.
- Contract rates and adjustments must be applied from the correct version and effective date.
- Call-off rules cannot be bypassed by requester preference.
- Supplier, line, period, and aggregate ceilings cannot be exceeded.
- A contract does not authorise purchase beyond valid demand or budget governance.
- Partial coverage and residual demand remain visible.
- Duplicate sourcing and commitment are prohibited.
- Historical rates, balances, decisions, and versions cannot be overwritten.
- Approval does not itself create a Purchase Order, liability, receipt, or payment.

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
| `ProcurementMethodDetermined` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `AwardRecommendationApproved` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ApprovedSourcingOutcomeFinalised` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| J03 | Supplies approved demand and remaining quantity. | Retains demand truth. |
| J02 | Supplies supplier eligibility. | Retains supplier truth. |
| Finance | Supplies budget, tax, and monetary references. | Retains monetary truth. |
| J04 | Consumes approved supplier, rate, quantity, and ceiling allocation. | Creates commitment. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J01 roles are separated as institutional policy requires: Procurement Officer; Procurement Manager; Requesting Unit Representative; Technical Evaluator; Commercial Evaluator; Committee Member and Committee Chair / Convenor where used; Procurement Recommender; Procurement Approver; External Procurement Coordinator where applicable; and Auditor / Reviewer. A requester, evaluator, recommender, or conflicted participant cannot replace the Procurement Approver decision.

## 15.2 Required Evidence

- demand approval, lines, quantity, location, required date, and budget reference;
- Active contract identity, version, line, supplier, applicability, validity, rate, charges, ceiling, and balance;
- supplier eligibility and call-off rule;
- mini-competition, matching, adjustment, exception, and residual evidence where applicable;
- recommendation, approval, actors, reasons, timestamps, and correlations.

## 15.3 Audit Obligations

Every outcome must trace from demand through the exact contract version, supplier rule, rate, ceiling, approval, and downstream allocation.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- an approved demand item is traceable to an active, eligible Rate Contract covering its item, category, currency, location, and institutional scope;
- the sourcing outcome and demand disposition are recorded with the governing Rate Contract identity and version;
- the consumed quantity or value is recorded against the Rate Contract ceiling, reservation, or ordered-to-date balance without exceeding authorised limits;
- compliance with pricing, terms, and ordering conditions of the Rate Contract is confirmed;
- the demand item's status reflects the Rate Contract sourcing outcome and is no longer available for re-sourcing;
- audit evidence linking the demand, the Rate Contract, and the ordering decision is preserved and reconstructable.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the demand item is sourced and its sourcing status reflects the governed Rate Contract outcome;
- the Rate Contract has an updated consumption or ordered-to-date record consistent with the ordered quantity or value;
- no new sourcing event, negotiation, or competing supplier decision is required for the covered demand;
- the sourcing outcome is available as a completed fact for downstream commitment, receipt, and payable scenarios;
- no Purchase Order, financial commitment, or payment obligation has been created by this scenario alone.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Whether expected delivery must fall within validity is institution-specific.
2. Choice, rotation, rank, mini-competition, ceiling reservation, and cancellation rules require operating-profile governance.
3. Boundary between sourcing utilisation and J04 committed utilisation requires final contract definition.

## 17.2 Assumptions

- Rate-contract sourcing approval is not a Purchase Order.
- Contract balances are interpreted according to authoritative business rules at the decision time.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-012**
