# Rate Contract Requirement to Approved Active Rate Contract

## 1. Specification Metadata

**Specification ID:** L4-J-011  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-011  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-011_Rate_Contract_Requirement_to_Approved_Active_Rate_Contract.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`  
**Referenced By:** L4-J-012 and future downstream L5 design  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern establishment of a rate contract from an authorised recurring or forecast procurement requirement through sourcing, evaluation, approval, and activation, while preserving the distinction between procurement availability and downstream purchase commitment.

The behavior begins when Procurement accepts the authorised requirement to establish a rate contract. It ends when the exact controlled Rate Contract version reaches `Active`, or when establishment is rejected, cancelled, returned, or directed to re-sourcing under the parent scenario.

This specification does not govern individual demand call-off, successor arrangements, suspension/closure scenarios, supplier-master lifecycle, Purchase Orders, receipts, payables, accounting, or payments.

---

## 3. Required Business Context

- An authorised institutional requirement and recurring-demand or forecast basis.
- Defined categories, scope/specifications, units, anticipated volume/value, locations, duration, and service expectations.
- Institution-governed sourcing method, competition, approval, duration, extension, ceiling, and call-off rules.
- Current J02 supplier identity, eligibility, qualification, suspension, and risk facts.
- J03 demand or forecast references where applicable without transfer of demand ownership.
- Controlled commercial terms including rates, units, taxes/charges, validity, ceilings, applicability, service conditions, and authorised adjustment/extension basis where applicable.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-011-BH-001 | When Procurement accepts the authorised rate-contract requirement, the system shall correlate the requirement, scope, forecast, locations, duration, anticipated value, and policy basis to the governed J01 sourcing activity. | L3-J-011 §§4.1, 7, 8, 9.1 |
| L4-J-011-BH-002 | Before supplier responses are evaluated, the system shall preserve the controlled sourcing basis for covered scope, units, rate format, price basis, taxes/charges, delivery/service conditions, validity, ceilings, call-off rules, and applicable extension or adjustment conditions. | L3-J-011 §§4.3, 9.4–9.6, 12 |
| L4-J-011-BH-003 | When suppliers participate in the sourcing activity, the system shall preserve their responses and the technical, commercial, capacity, risk, and service evaluation results used for the recommendation. | L3-J-011 §§9.7–9.8, 15.2 |
| L4-J-011-BH-004 | The system shall keep supplier-originated commercial terms distinguishable from evaluated, matched, negotiated, or approved commercial terms. | L3-J-011 §§9.9, 10.3, 12 |
| L4-J-011-BH-005 | For a multi-supplier arrangement, the system shall record each supplier's governed catalogue/scope, rank or selection position where applicable, location, ceiling, capacity, and call-off conditions under the approved sourcing rules. | L3-J-011 §§9.10–9.11, 10.2, 12 |
| L4-J-011-BH-006 | Before the arrangement can be approved, the system shall require current supplier eligibility and the complete controlled recommendation including suppliers, catalogue/scope, rates, taxes/charges, validity, ceilings, service conditions, adjustment basis, and exceptions where applicable. | L3-J-011 §§9.11–9.14, 12 |
| L4-J-011-BH-007 | When the Procurement Approver approves the exact controlled version, the system shall assign the Rate Contract its governed identity/version and recognise it in `Approved` without treating it as available for call-off until activation conditions are satisfied. | L3-J-011 §§9.14–9.16, 13 |
| L4-J-011-BH-008 | When the approved Rate Contract reaches its effective date and all parent-required activation prerequisites are satisfied, the system shall transition that controlled version to `Active`. | L3-J-011 §§3, 4.2, 9.16–9.17, 11.4, 13, 16 |
| L4-J-011-BH-009 | While a required activation prerequisite remains unsatisfied, the system shall prevent the approved Rate Contract from becoming `Active`. | L3-J-011 §11.4 |
| L4-J-011-BH-010 | The system shall prevent uncontrolled overlap with another Active arrangement covering the same scope unless authoritative rules explicitly permit and bound that overlap. | L3-J-011 §§7, 11.5 |
| L4-J-011-BH-011 | Once Active, the system shall make the Rate Contract available only within its approved suppliers, catalogue/scope, rates, validity, ceilings, applicability, and call-off conditions. | L3-J-011 §§3, 4.3, 9.17–9.18, 16 |
| L4-J-011-BH-012 | Activation shall not by itself create a Purchase Order, guaranteed consumption, payable, receipt, accounting entry, inventory/asset consequence, or payment. | L3-J-011 §§2, 4.3, 9.18, 12, 16 |
| L4-J-011-BH-013 | After activation, the system shall preserve the historical controlled version and shall not destructively rewrite it when later amendment, suspension, successor, or closure governance is required. | L3-J-011 §§11.7, 12 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-011-BH-014 | When the Rate Contract is created as the governed procurement arrangement, the system shall publish `RateContractCreated` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-011 §14.1; L2-J01 §35.10 |
| L4-J-011-BH-015 | When the exact controlled Rate Contract version is authoritatively approved, the system shall publish `RateContractApproved` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-011 §14.1; L2-J01 §35.10 |
| L4-J-011-BH-016 | When the approved Rate Contract becomes `Active`, the system shall publish `RateContractActivated` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-011 §14.1; L2-J01 §§27.3, 35.10 |

J02 retains supplier truth; J03 retains procurement-demand truth; Finance retains financial truth; J04 alone creates Purchase Order commitment.

---

## 6. Failure / Rejection / Continuation

- Invalid requirement authority, improper aggregation, invalid sourcing method, or artificial fragmentation shall prevent establishment from progressing until a source-supported correction or re-sourcing outcome exists.
- A supplier that becomes ineligible before activation shall not be activated; only the parent-supported reallocation, reduced activation, or re-sourcing path may continue.
- Ambiguous or inconsistent commercial schedule facts shall prevent approval or activation until corrected and re-approved while preserving prior versions.
- Missing acceptance, security, insurance, licence, document, or other parent-required activation evidence shall keep the contract `Approved` but not `Active`, or permit only the rejection path supported upstream.
- An interrupted approval shall leave the arrangement unapproved and unavailable for call-off.
- A post-activation error shall not cause destructive rewrite of the Active historical version; later governed amendment, suspension, successor, or closure remains outside this scenario.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- the authorised requirement, forecast, category/scope, value, locations, duration, and policy basis;
- sourcing method and issued sourcing conditions;
- supplier responses, eligibility, evaluation, competition, and commercial reasonableness evidence;
- covered catalogue/scope, units, rates, taxes/charges, discounts, validity, ceilings, locations, service levels, call-off rules, and applicable adjustment/extension basis;
- original, evaluated, matched, negotiated, and approved terms where applicable;
- activation prerequisites and their satisfaction evidence where applicable;
- recommendation, review, approval/return/rejection decision, effective date, actor, authority, reason, and controlled version;
- `RateContractCreated`, `RateContractApproved`, and `RateContractActivated` business-event evidence.

---

## 8. Observable Completion Conditions

Successful completion is observable only when:

1. the requirement, aggregate value, sourcing method, and authority are valid and traceable;
2. supplier eligibility and sourcing evaluation are complete;
3. the exact controlled version contains the approved suppliers, scope/catalogue, units, rates, taxes/charges, validity, ceilings, applicability, service conditions, and call-off rules required by the parent;
4. the Procurement Approver has approved that exact controlled version;
5. all required activation prerequisites and the effective-date condition are satisfied;
6. the Rate Contract reaches `Active` and `RateContractActivated` is published;
7. activation creates availability only and no Purchase Order or downstream financial/physical outcome is created by this scenario alone.

Rejected, cancelled, returned, or re-sourcing outcomes are valid terminal alternatives only as supported by the parent.

---

## 9. Traceability & Provenance

**Parent:** L3-J-011 — Rate Contract Requirement to Approved Active Rate Contract  
**Primary verification source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Additional verification sources:** L2-J00 — Enterprise Resource Management Architecture & Governance; L2-J02 — Vendor & Supplier Management Workflows; L2-J03 — Purchase Requisition & Approval Workflows; locked L1 Business Domain Architecture.  

Every BH is anchored in the Locked parent. L2 sources verify J01 ownership, the `Draft → Review → Approved → Active` Rate Contract lifecycle segment, supplier/demand ownership boundaries, and exact J01 rate-contract event names.

---

## 10. Open Issues / Deferred Design

- Institution-specific limits for contract duration, extension, ceilings, minimum commitment, rate matching, rotation, mini-competition, price adjustment, tax change, and indexation remain upstream-governed and are not universalised at L4.
- The parent records an unresolved boundary question between procurement rate contract, legal contract execution, framework agreement/empanelment, and J04 call-off. This L4 does not resolve or redefine that boundary.
- The parent lists `Section_J_Canonical_Register.md` as a dependency, but that exact file was not retrievable through the current repository search. No L4 behavior has been derived from an assumed replacement; this is retained as a repository verification finding.
- API/interface design, persistence/data model, component/service design, event transport, technical access control, transaction/retry mechanics, infrastructure, and UI/UX remain deferred to L5/L6.
